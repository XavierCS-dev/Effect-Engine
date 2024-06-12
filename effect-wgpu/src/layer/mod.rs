use anyhow::Result;
use effect_core::{
    id::{LayerID, TextureID},
    primitives::vertex::Vertex,
    raw::entityraw::Entity2DRaw,
};
use std::collections::{hash_map::Keys, HashMap};
use winit::dpi::PhysicalSize;

use crate::{
    allocator::BufferAllocator,
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
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    entity_count: usize,
    entity_maximum: usize,
    entity_buffer: Option<wgpu::Buffer>,
}

impl Layer2D {
    pub fn new(
        id: LayerID,
        texture_dimensions: PhysicalSize<u32>,
        textures: Vec<TextureDescriptor2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let texture_array = TextureArray::new(device, queue, textures, texture_dimensions)?;
        // use separate new system for managing buffers.
        let entity_count = 0;
        let entity_maximum = 0;
        let entity_buffer = None;
        let vertex_buffer = BufferAllocator::default()
            .usage(wgpu::BufferUsages::VERTEX)
            .size(std::mem::size_of::<Vertex>() as u64 * 4)
            .allocate(device);
        let index_buffer = BufferAllocator::default()
            .usage(wgpu::BufferUsages::INDEX)
            .size(std::mem::size_of::<u32>() as u64 * 6)
            .allocate(device);

        panic!("Vertex and index buffers do not have their data assigned");

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
