use anyhow::{bail, Result};
use std::collections::{hash_map::Keys, HashMap};

use wgpu::util::DeviceExt;

use crate::{
    engine::{
        entity::entity::{Entity2D, Entity2DRaw},
        primitives::vertex::Vertex,
        texture::{
            texture2d::{Texture2D, TextureID},
            texture_atlas2d::TextureAtlas2D,
        },
        util::effect_error::EffectError,
    },
    EffectSystem,
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct LayerID(pub u32);

// Takes final ownership of textures, the data etc.
// When a entity wants to get the texture offset, it must get the data from here.
pub struct Layer2D {
    id: LayerID,
    textures: HashMap<TextureID, Texture2D>,
    atlas: Option<TextureAtlas2D>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    entity_count: usize,
    entity_maximum: usize,
    entity_buffer: Option<wgpu::Buffer>,
}

impl Layer2D {
    pub fn new(id: LayerID) -> Result<Self> {
        let mut textures = HashMap::new();
        let index_buffer = None;
        let entity_count = 0;
        let entity_maximum = 0;
        let atlas = None;
        Ok(Self {
            id,
            textures,
            atlas,
            vertex_buffer: None,
            index_buffer,
            entity_count,
            entity_buffer: None,
            entity_maximum,
        })
    }

    pub fn id(&self) -> LayerID {
        self.id
    }

    pub fn contains_texture(&self, texture_id: &TextureID) -> bool {
        self.textures.contains_key(texture_id)
    }

    pub fn texture_ids<'a>(&'a self) -> Keys<'a, TextureID, Texture2D> {
        self.textures.keys()
    }

    // if using the 2x technique, its probably better to return the exact slice where the data is
    // instead of the whole buffer, same for the other buffers
    // 4 is the number of vertices per entity
    pub fn vertex_buffer(&self) -> Option<&wgpu::BufferSlice> {
        match self.vertex_buffer {
            Some(v_buf) => {
                let length = self.entity_count * std::mem::size_of::<[f32; 3]>() * 4;
                Some(&v_buf.slice(0..length as u64))
            }
            None => None,
        }
    }

    // 6 is the number of indicies per entity.
    pub fn index_buffer(&self) -> Option<&wgpu::BufferSlice> {
        match self.index_buffer {
            Some(i_buf) => {
                let length = self.entity_count * std::mem::size_of::<u16>() * 6;
                Some(&i_buf.slice(0..length as u64))
            }
            None => None,
        }
    }

    pub fn entity_buffer(&self) -> Option<&wgpu::BufferSlice> {
        match self.entity_buffer {
            Some(e_buf) => {
                let length = self.entity_count * Entity2DRaw::size();
                Some(&e_buf.slice(0..length as u64))
            }
            None => None,
        }
    }

    pub fn index_count(&self) -> usize {
        self.entity_count * 6
    }

    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    pub fn entity_maximum(&self) -> usize {
        self.entity_maximum
    }

    pub fn get_texture(&self, id: TextureID) -> Option<&Texture2D> {
        self.textures.get(&id)
    }
}

pub struct Layer2DSystem;

impl Layer2DSystem {
    fn alloc_buffer(
        data: &[u8],
        size: u64,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: &str,
    ) -> wgpu::Buffer {
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size: size * 2,
            usage: wgpu::BufferUsages::VERTEX,
            mapped_at_creation: false,
        });
        queue.write_buffer(&buffer, 0, data);
        buffer
    }

    // alloc buffer to 2x the size and set new max entity count
    fn create_entity_buffer(
        entities: &[&Entity2D],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::Buffer {
        let data: &[u8] = bytemuck::cast_slice(
            entities
                .iter()
                .map(|e| e.to_raw())
                .collect::<Vec<_>>()
                .as_slice(),
        );
        let size = std::mem::size_of_val(data) as u64;
        Layer2DSystem::alloc_buffer(data, size, device, queue, "Entity Buffer")
    }

    fn create_index_buffer(
        device: &wgpu::Device,
        entity_count: usize,
        queue: &wgpu::Queue,
    ) -> wgpu::Buffer {
        let mut indices = Vec::new();
        indices.reserve(std::mem::size_of::<u16>() * entity_count as usize);
        for _ in 0..entity_count {
            indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
        }
        let data: &[u8] = bytemuck::cast_slice(&indices);
        let size = (entity_count * std::mem::size_of::<u16>()) as u64;
        Layer2DSystem::alloc_buffer(data, size, device, queue, "Index Buffer")
    }

    fn create_vertex_buffer(
        entities: &[&Entity2D],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::Buffer {
        let data: &[u8] = bytemuck::cast_slice(
            entities
                .iter()
                .flat_map(|e| e.vertices())
                .copied()
                .collect::<Vec<_>>()
                .as_slice(),
        );
        let size = std::mem::size_of_val(data) as u64;
        Layer2DSystem::alloc_buffer(data, size, device, queue, "Vertex Buffer")
    }

    /// Update transformation data (not the vertices).
    // Panics if the number of entities changed
    pub fn update_entities(layer: &mut Layer2D, entities: &[&Entity2D], queue: &wgpu::Queue) {
        if entities.len() != layer.entity_count() {
            panic!("Entities would not fit buffer")
        }
        let data: Vec<Entity2DRaw> = entities.iter().map(|e| e.to_raw()).collect();
        queue.write_buffer(
            &layer.entity_buffer.unwrap(),
            0,
            bytemuck::cast_slice(&data),
        );
    }

    /// Set the vertices and entity data. Use this when adding or removing entities
    pub fn set_entities(
        layer: &mut Layer2D,
        entities: &[&Entity2D],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // allocating exactly amount needed each time may increase the number of allocations needed..
        // perhaps a strategy of allocatin 2X needed data would be better
        layer.entity_count = entities.len();

        if layer.entity_count() > layer.entity_maximum() || layer.vertex_buffer().is_none() {
            // Allocate new buffers
            layer.entity_buffer = Some(Layer2DSystem::create_entity_buffer(
                &entities, device, queue,
            ));
            layer.vertex_buffer = Some(Layer2DSystem::create_vertex_buffer(
                &entities, device, queue,
            ));
            layer.index_buffer = Some(Layer2DSystem::create_index_buffer(
                device,
                layer.entity_count(),
                queue,
            ));
            layer.entity_maximum = layer.entity_count * 2;
        } else {
            // Reuse buffers
            let entity_data: &[u8] = bytemuck::cast_slice(
                entities
                    .iter()
                    .map(|e| e.to_raw())
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
            let vertex_data: &[u8] = bytemuck::cast_slice(
                entities
                    .iter()
                    .flat_map(|e| e.vertices())
                    .copied()
                    .collect::<Vec<_>>()
                    .as_slice(),
            );
            let mut indices = Vec::new();
            indices.reserve(std::mem::size_of::<u16>() * layer.entity_count());
            for _ in 0..layer.entity_count() {
                indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
            }
            let index_data: &[u8] = bytemuck::cast_slice(&indices);
            queue.write_buffer(&layer.entity_buffer.unwrap(), 0, entity_data);
            queue.write_buffer(&layer.vertex_buffer.unwrap(), 0, vertex_data);
            queue.write_buffer(&layer.index_buffer.unwrap(), 0, index_data);
        }
    }
}
