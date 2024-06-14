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
    entities: usize,
    entity_buffer: Option<Buffer>,
}

const INDICES: [u16; 6] = [0, 1, 2, 0, 2, 3];

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
        let entities = 0;
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
        let index_buffer = BufferAllocator::default()
            .usage(wgpu::BufferUsages::INDEX)
            .data(Vec::from(bytemuck::cast_slice(&INDICES)))
            .allocate(device);

        Ok(Self {
            id,
            texture_array,
            vertex_buffer,
            index_buffer,
            entities,
            entity_buffer,
        })
    }

    pub fn id(&self) -> LayerID {
        self.id
    }

    pub fn entity_count(&self) -> usize {
        self.entities
    }

    pub fn get_texture(&self, id: &TextureID) -> Option<Texture2D> {
        self.texture_array.texture(id)
    }

    pub fn texture_dimensions(&self) -> PhysicalSize<u32> {
        self.texture_array.texture_dimensions()
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        self.texture_array.bind_group()
    }

    pub fn get_textures(&self) -> &HashMap<TextureID, Texture2D> {
        self.texture_array.textures()
    }

    pub fn vertex_buffer(&self) -> wgpu::BufferSlice {
        self.vertex_buffer.buffer()
    }

    pub fn index_buffer(&self) -> wgpu::BufferSlice {
        self.index_buffer.buffer()
    }

    pub fn entity_buffer(&self) -> Option<wgpu::BufferSlice> {
        match self.entity_buffer.as_ref() {
            Some(buf) => Some(buf.buffer()),
            None => None,
        }
    }
}

pub struct Layer2DSystem;

impl Layer2DSystem {
    pub fn set_entities(
        layer: &mut Layer2D,
        entities: &[&Entity2D],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let data: Vec<Entity2DRaw> = entities.iter().map(|e| e.to_raw()).collect();
        match layer.entity_buffer.as_mut() {
            Some(buf) => {
                buf.write(bytemuck::cast_slice(&data), device, queue);
            }
            None => {
                // Not using the data method as it is better for when the entities won't incease,
                // as reallocating a lot when creating new layers / extending the buffer could cause stutters.
                // the *2 means 2 times the needed buffer size will be allocated
                let mut buf = BufferAllocator::default()
                    .usage(wgpu::BufferUsages::VERTEX)
                    .size((std::mem::size_of::<Entity2DRaw>() * data.len()) as u64 * 2)
                    .allocate(device);
                buf.write(bytemuck::cast_slice(&data), device, queue);
                layer.entity_buffer = Some(buf);
            }
        }
        // Extend indices to match number of entities
        layer.entities = entities.len();
        let mut data = Vec::new();
        for i in 0..entities.len() {
            data.extend_from_slice(&INDICES);
        }
        layer
            .index_buffer
            .write(bytemuck::cast_slice(&data), device, queue);
    }
}
