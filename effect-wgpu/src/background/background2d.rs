use effect_core::primitives::vertex::Vertex;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use anyhow::Result;

use crate::texture::texture2d::{WebTexture2D, WebTexture2DSystem};

// Render before layers.
// Support parallax in the future.
// should just request bind groups etc and should just work
// including any animations etc
pub struct WebBackground2D {
    bind_group: wgpu::BindGroup,
    texture_data: WebTexture2D,
    dimensions: PhysicalSize<u32>,
    vertex_buffer: wgpu::Buffer,
    camera_spoof: wgpu::Buffer,
    entity_spoof: wgpu::Buffer,
    camera_bind_group: wgpu::BindGroup,
}

// Switching render pipelines and shader is very costly just for background...
// texture buffer switches for every layer anyway, so only buffer affected is
// is camera buffer which is only switched out once.
#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
pub struct EntitySpoof {
    transform: [[f32; 4]; 4],
    texture_index: [f32; 2],
    texture_size: [f32; 2],
}

impl WebBackground2D {
    pub fn new(
        texture: WebTexture2D,
        bind_group_layout: &wgpu::BindGroupLayout,
        pixel_art: bool,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let tex = image::open(texture.file_path())?;
        let dimensions = PhysicalSize::new(tex.width(), tex.height());
        let tex_rgb = tex.into_rgba8();
        let extent = wgpu::Extent3d {
            width: dimensions.width,
            height: dimensions.height,
            depth_or_array_layers: 1,
        };
        let mut texture = texture;
        WebTexture2DSystem::set_index(&mut texture, [0, 0]);
        WebTexture2DSystem::set_dimensions(&mut texture, dimensions.width, dimensions.height);

        let bind_group = WebTexture2DSystem::init_texture(
            extent,
            tex_rgb,
            bind_group_layout,
            pixel_art,
            device,
            queue,
        );

        let entity_spoof = EntitySpoof {
            transform: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            texture_index: [0.0, 0.0],
            texture_size: [0.0, 0.0],
        };

        // Rust automatically assumes f64
        let camera_spoof = [
            [1.0f32, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];

        let vertices = [
            Vertex {
                position: [1.0, 1.0, 0.0],
                tex_coords: [1.0, 0.0],
            },
            Vertex {
                position: [-1.0, 1.0, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-1.0, -1.0, 0.0],
                tex_coords: [0.0, 1.0],
            },
            Vertex {
                position: [1.0, -1.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ];

        let entity_spoof = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Entity spoof"),
            contents: bytemuck::cast_slice(&[entity_spoof]),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let camera_spoof = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera spoof"),
            contents: bytemuck::cast_slice(&camera_spoof),
            usage: wgpu::BufferUsages::UNIFORM,
        });

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Background"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_spoof.as_entire_binding(),
            }],
        });

        Ok(Self {
            bind_group,
            texture_data: texture,
            dimensions,
            entity_spoof,
            camera_spoof,
            vertex_buffer,
            camera_bind_group,
        })
    }

    pub fn vertex_buffer(&self) -> wgpu::BufferSlice {
        self.vertex_buffer.slice(..)
    }

    pub fn camera_buffer(&self) -> wgpu::BufferSlice {
        self.camera_spoof.slice(..)
    }

    pub fn entity_buffer(&self) -> wgpu::BufferSlice {
        self.entity_spoof.slice(..)
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn camera_bind_group(&self) -> &wgpu::BindGroup {
        &self.camera_bind_group
    }

    pub fn texture(&self) -> &WebTexture2D {
        &self.texture_data
    }

    pub fn dimensions(&self) -> PhysicalSize<u32> {
        self.dimensions
    }
}

// manage all background related operations
pub struct WebBackground2DSystem;
