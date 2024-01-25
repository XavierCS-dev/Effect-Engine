use crate::engine::entity::entity::Entity2D;
use crate::engine::entity::entity::Entity2DRaw;
use crate::engine::layer::layer::*;
use wgpu::util::DeviceExt;

use super::{
    primitives::vertex::Vertex,
    texture::{
        texture2d::{Texture2D, TextureID},
        texture_pool::TexturePool2D,
    },
    traits::layer::Layer,
};

pub struct Engine {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_configuration: wgpu::SurfaceConfiguration,
    window: winit::window::Window,
    render_pipeline: wgpu::RenderPipeline,
    texture_pool: TexturePool2D,
}

/*
* Update and input should run closures defined by the user
* these closures are to be stored in Engine upon initialisation
*/
impl Engine {
    pub async fn new(window: winit::window::Window) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let surface = unsafe { instance.create_surface(&window).unwrap() };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("Adapter"),
                    features: adapter.features(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();

        let texture_pool = TexturePool2D::new(&device);
        let surface_capabilities = surface.get_capabilities(&adapter);
        // Check this...may need to specifically set it to some sRGB value
        let surface_format = surface_capabilities.formats[0];
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
        };
        surface.configure(&device, &surface_configuration);

        let shader_module =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline layout descriptor"),
                bind_group_layouts: &[texture_pool.bind_group_layout()],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
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
                module: &shader_module,
                entry_point: "frg_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_configuration.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            surface_configuration,
            window,
            render_pipeline,
            texture_pool,
        }
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        if size.width > 0 && size.height > 0 {
            self.surface_configuration.width = size.width;
            self.surface_configuration.height = size.height;
            self.surface
                .configure(&self.device, &self.surface_configuration);
        }
    }

    pub fn input(&mut self, event: &winit::event::Event<()>, delta: &std::time::Duration) {
        // do nothing
        // not sure what to do with this yet
        // TODO: move input to be a burden on user.
    }

    pub fn update(&mut self, delta: &std::time::Duration) {
        // TODO: Move update to be a burden on user

        // millis returns 0 for some reason...use nano
        // if accuracy is a problem, change to floats
    }

    // Must be rendered in order. Maybe be no existent layers inbetween, needs to happen fast.
    // Vec can be sorted each time entities is appended to with a new entity2d and layerid.
    // probably a lot faster and space efficient than hashmap
    // entities managed by APP struct
    pub fn render(&mut self, entities: Vec<Vec<&Entity2D>>) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = self.surface.get_current_texture()?;
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command encoder"),
                });
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.4,
                        b: 0.5,
                        a: 0.3,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        for (layer_id, layer) in self.texture_pool.get_layers() {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(
                0,
                self.texture_pool.get_layer(layer_id).unwrap().bind_group(),
                &[],
            );
            render_pass.set_vertex_buffer(0, layer.vertex_buffer().unwrap().slice(..));
            render_pass.set_index_buffer(
                layer.index_buffer().unwrap().slice(..),
                wgpu::IndexFormat::Uint32,
            );
            render_pass.draw_indexed(0..layer.index_count() as u32, 0, 0..1);
        }

        drop(render_pass);
        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        Ok(())
    }
}
