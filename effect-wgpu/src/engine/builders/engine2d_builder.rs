use std::{borrow::Cow, collections::BTreeMap, fs::File, io::BufReader, sync::Arc};

use effect_core::{
    id::LayerID, misc::window_info::WindowInfo, primitives::vertex::Vertex,
    raw::entityraw::Entity2DRaw,
};
use effect_util::file_to_bytes;
use wgpu::{naga::back, util::DeviceExt, PowerPreference};
use winit::dpi::PhysicalSize;

use crate::{
    background::background2d::Background2D,
    camera::Camera,
    engine::engine2d::Engine2D,
    entity::entity2d::Entity2DLayout,
    layer::Layer2D,
    layouts::VertexLayout,
    texture::texture2d::{Texture2D, Texture2DBGL},
    window::Window,
};

pub struct Engine2DBuilder {
    window: Option<winit::window::Window>,
    window_info: WindowInfo,
    power_preference: wgpu::PowerPreference,
    bind_group_layouts: Vec<wgpu::BindGroupLayoutDescriptor<'static>>,
    vertex_shader: Option<&'static str>, // option to detect if shader was set at all vs wrong path
    fragment_shader: Option<&'static str>,
}

impl Default for Engine2DBuilder {
    fn default() -> Self {
        let window = None;
        let power_preference = wgpu::PowerPreference::HighPerformance;
        let bind_group_layouts = Vec::new();
        let vertex_shader = None;
        let fragment_shader = None;
        let window_info = WindowInfo::default();
        Self {
            window,
            window_info,
            power_preference,
            bind_group_layouts,
            vertex_shader,
            fragment_shader,
        }
    }
}

impl Engine2DBuilder {
    pub fn window(mut self, window: winit::window::Window) -> Self {
        self.window = Some(window);
        self
    }

    pub fn window_info(mut self, window_info: WindowInfo) -> Self {
        self.window_info = window_info;
        self
    }

    pub fn vertex_shader(mut self, path: &'static str) -> Self {
        self.vertex_shader = Some(path);
        self
    }

    pub fn fragment_shader(mut self, path: &'static str) -> Self {
        self.fragment_shader = Some(path);
        self
    }

    pub fn power_preference(mut self, preference: wgpu::PowerPreference) -> Self {
        self.power_preference = preference;
        self
    }

    pub fn bind_group_layouts(
        mut self,
        layouts: Vec<wgpu::BindGroupLayoutDescriptor<'static>>,
    ) -> Self {
        self.bind_group_layouts = layouts;
        self
    }

    pub async fn build<'a>(self) -> Engine2D<'a> {
        let window = Arc::new(self.window.expect("Window must be supplied"));
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).unwrap();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: self.power_preference,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Adapter"),
                    required_features: adapter.features(),
                    required_limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let surface_capabilities = surface.get_capabilities(&adapter);
        let present_mode;
        if self.window_info.vsync {
            present_mode = wgpu::PresentMode::AutoVsync;
        } else {
            present_mode = wgpu::PresentMode::AutoNoVsync;
        }

        let surface_format = surface_capabilities.formats[0];
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: self.window_info.resolution.width,
            height: self.window_info.resolution.width,
            present_mode,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: Default::default(),
        };
        surface.configure(&device, &surface_config);
        let vrt_path = self.vertex_shader.expect("Vertex shader path must be set.");
        let frg_path = self
            .fragment_shader
            .expect("Fragment shader path must be set");
        let vert_module = device.create_shader_module(wgsl_shader_builder(vrt_path));
        let frag_module = device.create_shader_module(wgsl_shader_builder(frg_path));

        let mut bind_group_layouts = Vec::new();
        for layout in self.bind_group_layouts.iter() {
            bind_group_layouts.push(device.create_bind_group_layout(layout));
        }

        let mut bgls = Vec::new();
        for layout in bind_group_layouts.iter() {
            bgls.push(layout);
        }
        let graphics_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &bgls,
                push_constant_ranges: &[],
            });

        let graphics_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&graphics_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vert_module,
                entry_point: "vrt_main",
                buffers: &[Vertex::layout(), Entity2DRaw::layout()],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: if frg_path == vrt_path {
                    &vert_module
                } else {
                    &frag_module
                },
                entry_point: "frg_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let background: Option<Background2D> = None;
        let proj = glam::Mat4::perspective_rh(
            45.0f32.to_radians(),
            self.window_info.resolution.width as f32 / self.window_info.resolution.height as f32,
            0.1,
            10.0,
        );
        let look_at = glam::Mat4::look_at_rh(
            glam::Vec3::new(0.0f32, 0.0, 1.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::Y,
        );
        let camera = Camera::new(&device, proj, look_at);
        let layers: BTreeMap<LayerID, Layer2D> = BTreeMap::new();
        let window = Window::new(window, surface, surface_config, self.window_info.fullscreen);
        let texture_bgl = device.create_bind_group_layout(&Texture2D::layout());
        Engine2D::new(
            device,
            queue,
            window,
            graphics_pipeline,
            texture_bgl,
            background,
            index_buffer,
            camera,
            BTreeMap::new(),
        )
    }
}

pub fn wgsl_shader_builder(path: &'static str) -> wgpu::ShaderModuleDescriptor {
    let data = unsafe { String::from_utf8_unchecked(file_to_bytes(path)) };
    let data = Cow::Owned(data);

    wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(data),
    }
}
