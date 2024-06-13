use anyhow::Result;
use effect_core::{
    id::{LayerID, TextureID},
    primitives::vertex::Vertex,
    raw::entityraw::Entity2DRaw,
};
use std::collections::{hash_map::Keys, HashMap};
use winit::dpi::PhysicalSize;

use crate::{
    allocator::{Buffer, BufferAllocator},
    entity::entity2d::Entity2D,
    texture::{
        texture2d::{Texture2D, TextureDescriptor2D},
        texture_array::TextureArray,
    },
};

// Takes final ownership of textures, the data etc.
// When a entity wants to get the texture offset, it must get the data from here.
pub struct Layer2D {
    id: LayerID,
    texture_array: TextureArray,
    // vertex buffer should be replaced by vertex generation in the shader
    vertex_buffer: Buffer,
    index_buffer: Buffer,
    entity_count: usize,
    entity_maximum: usize,
    entity_buffer: Option<Buffer>,
}

impl Layer2D {
    pub fn new(
        id: LayerID,
        texture_dimensions: PhysicalSize<u32>,
        screen_dimensions: PhysicalSize<u32>,
        textures: Vec<TextureDescriptor2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let texture_array = TextureArray::new(device, queue, textures, texture_dimensions)?;
        // use separate new system for managing buffers.
        let entity_count = 0;
        let entity_maximum = 0;
        let entity_buffer = None;
        // to maintain aspect ratio, divide both by width
        let width = (texture_dimensions.width / screen_dimensions.width) as f32;
        let height = (texture_dimensions.height / screen_dimensions.width) as f32;
        let vertices = [
            Vertex {
                position: [width, height, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.0, height, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
            Vertex {
                position: [width, 0.0, 0.0],
                tex_coords: [1.0, 1.0],
            },
        ];
        let vertex_buffer = BufferAllocator::default()
            .usage(wgpu::BufferUsages::VERTEX)
            .data(Vec::from(bytemuck::cast_slice(&vertices)))
            .allocate(device);
        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = BufferAllocator::default()
            .usage(wgpu::BufferUsages::INDEX)
            .data(Vec::from(bytemuck::cast_slice(&indices)))
            .allocate(device);

        Ok(Self {
            id,
            texture_array,
            vertex_buffer,
            index_buffer,
            entity_count,
            entity_maximum,
            entity_buffer,
        })
    }
}

pub struct Layer2DSystem;

impl Layer2DSystem {}
