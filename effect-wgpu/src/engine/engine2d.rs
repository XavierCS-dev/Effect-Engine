use std::{collections::BTreeMap, sync::Arc, time::Duration};

use effect_core::{
    camera::camera2d::{Camera2D, Camera2DSystem},
    id::{LayerID, TextureID},
    primitives::{vector::Vector3, vertex::Vertex},
    raw::entityraw::Entity2DRaw,
};
use effect_events::input::EffectEvent;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use anyhow::Result;

use crate::{
    background::background2d::WebBackground2D,
    camera::{WebCamera, WebCameraSystem2D},
    entity::entity2d::{WebEntity2D, WebEntity2DRaw},
    layer::{WebLayer2D, WebLayer2DSystem},
    layouts::WebVertexLayout,
    texture::texture2d::WebTexture2D,
};

pub struct WebEngine2D {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_configuration: wgpu::SurfaceConfiguration,
    pub window: Arc<winit::window::Window>,
    render_pipeline: wgpu::RenderPipeline,
    texture_bgl: wgpu::BindGroupLayout,
    background: Option<WebBackground2D>,
    index_buffer: wgpu::Buffer,
    camera: WebCamera,
    pub layers: BTreeMap<LayerID, WebLayer2D>,
}

/*
* Update and input should run closures defined by the user
* these closures are to be stored in Engine upon initialisation
* 0.3.0 release
*/
impl WebEngine2D {
    pub async fn new(window: winit::window::Window, v_sync: bool) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        let window = Arc::new(window);
        let surface = instance.create_surface(window.clone()).unwrap();
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
        if v_sync {
            present_mode = wgpu::PresentMode::AutoVsync;
        } else {
            present_mode = wgpu::PresentMode::AutoNoVsync;
        }
        // Check this...may need to specifically set it to some sRGB value
        let surface_format = surface_capabilities.formats[0];
        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: Vec::new(),
            desired_maximum_frame_latency: Default::default(),
        };
        surface.configure(&device, &surface_configuration);

        let shader_module =
            device.create_shader_module(wgpu::include_wgsl!("../shaders/shader.wgsl"));

        let texture_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
            ],
            label: Some("Bind group layout"),
        });

        let camera_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Pipeline layout descriptor"),
                bind_group_layouts: &[&texture_bgl, &camera_bgl],
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
                    blend: Some(wgpu::BlendState::PREMULTIPLIED_ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        let background = None;

        let proj = glam::Mat4::perspective_rh(
            45.0f32.to_radians(),
            window.inner_size().width as f32 / window.inner_size().height as f32,
            0.1,
            10.0,
        );
        let look_at = glam::Mat4::look_at_rh(
            glam::Vec3::new(0.0f32, 0.0, 1.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::Y,
        );
        let camera = WebCamera::new(&device, proj, look_at);

        let layers = BTreeMap::new();

        Self {
            surface,
            device,
            queue,
            surface_configuration,
            window,
            render_pipeline,
            texture_bgl,
            background,
            index_buffer,
            camera,
            layers,
        }
    }

    pub fn resize(
        &mut self,
        size: winit::dpi::PhysicalSize<u32>,
        camera: &mut Option<&mut Camera2D>,
    ) {
        if size.width > 0 && size.height > 0 {
            self.surface_configuration.width = size.width;
            self.surface_configuration.height = size.height;
            self.surface
                .configure(&self.device, &self.surface_configuration);
            match camera.as_mut() {
                Some(camera) => {
                    Camera2DSystem::update_projection(camera, self.window.inner_size());
                    WebCameraSystem2D::update(camera, &mut self.camera);
                }
                _ => {
                    WebCameraSystem2D::update_projection(
                        &mut self.camera,
                        self.window.inner_size(),
                    );
                }
            };
            WebCameraSystem2D::update_buffers(&self.camera, &self.queue)
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
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
                        r: 0.0,
                        g: 0.5,
                        b: 0.5,
                        a: 0.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        match self.background.as_ref() {
            Some(bg) => {
                render_pass.set_bind_group(0, bg.bind_group(), &[]);
                render_pass.set_bind_group(1, bg.camera_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, bg.vertex_buffer());
                render_pass.set_vertex_buffer(1, bg.entity_buffer());
                render_pass.draw_indexed(0..6, 0, 0..1);
            }
            None => (),
        };

        render_pass.set_bind_group(1, self.camera.bind_group(), &[]);
        for (_, layer) in self.layers.iter() {
            render_pass.set_bind_group(0, layer.bind_group(), &[]);
            render_pass.set_vertex_buffer(0, layer.vertex_buffer());
            render_pass.set_vertex_buffer(1, layer.entity_buffer().unwrap());
            render_pass.draw_indexed(0..6 as u32, 0, 0..layer.entity_count() as u32);
        }
        drop(render_pass);
        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        Ok(())
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn init_layer(
        &mut self,
        id: LayerID,
        textures: Vec<WebTexture2D>,
        texture_size: PhysicalSize<u32>,
        pixel_art: bool,
    ) -> Result<()> {
        let layer = WebLayer2D::new(
            id,
            self.window.inner_size(),
            textures,
            &self.device,
            &self.queue,
            &self.texture_bgl,
            texture_size,
            pixel_art,
        )?;

        let _ = self.layers.insert(id, layer);

        Ok(())
    }

    pub fn init_entity(
        &self,
        position: Vector3<f32>,
        layer: LayerID,
        texture_id: TextureID,
    ) -> WebEntity2D {
        WebEntity2D::new(position, self.layers.get(&layer).unwrap(), texture_id)
    }

    pub fn set_entities(&mut self, layer: LayerID, entities: &[&WebEntity2D]) {
        WebLayer2DSystem::set_entities(
            self.layers.get_mut(&layer).unwrap(),
            entities,
            &self.device,
            &self.queue,
        )
    }

    pub fn update_camera(
        &mut self,
        camera: &mut Camera2D,
        ctx: &EffectEvent,
        delta_time: Duration,
    ) {
        Camera2DSystem::update(camera, ctx, delta_time);
        WebCameraSystem2D::update(camera, &mut self.camera);
        WebCameraSystem2D::update_buffers(&self.camera, &self.queue)
    }

    pub fn init_camera(&self, fov: f32) -> Camera2D {
        let dims = self.window.inner_size();
        Camera2D::new(fov, (dims.width as f32) / (dims.height as f32), 0.5)
    }

    pub fn set_background(&mut self, texture: WebTexture2D, pixel_art: bool) -> Result<()> {
        self.background = Some(WebBackground2D::new(
            texture,
            &self.texture_bgl,
            pixel_art,
            &self.device,
            &self.queue,
        )?);
        Ok(())
    }
}
