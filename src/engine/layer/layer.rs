use anyhow::Result;
use std::collections::HashMap;

use wgpu::util::DeviceExt;

use crate::engine::{
    entity::entity::{Entity2D, Entity2DRaw},
    primitives::vertex::Vertex,
    texture::{
        texture2d::{Texture2D, TextureID},
        texture_atlas2d::TextureAtlas2D,
    },
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct LayerID(pub u32);

pub struct Layer2D {
    id: LayerID,
    textures: HashMap<TextureID, Texture2D>,
    atlas: Option<TextureAtlas2D>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    entity_count: usize,
    entity_buffer: Option<wgpu::Buffer>,
}

impl Layer2D {
    pub fn new(id: LayerID) -> Result<Self> {
        let mut textures = HashMap::new();
        let index_buffer = None;
        let entity_count = 0;
        Ok(Self {
            id,
            textures,
            atlas: None,
            vertex_buffer: None,
            index_buffer,
            entity_count,
            entity_buffer: None,
        })
    }

    pub fn id(&self) -> LayerID {
        self.id
    }

    pub fn contains_texture(&self, texture_id: &TextureID) -> bool {
        self.textures.contains_key(texture_id)
    }

    /// Add a texture to a layer for entities to use
    // set offset of all textures when added
    pub fn add_texture(
        &mut self,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<()> {
        todo!()
    }

    pub fn get_texture(&self, texture_id: &TextureID) -> Option<&Texture2D> {
        self.textures.get(texture_id)
    }

    pub fn get_texture_mut(&mut self, texture_id: &TextureID) -> Option<&mut Texture2D> {
        self.textures.get_mut(texture_id)
    }

    pub fn bind_group(&self) -> Option<&wgpu::BindGroup> {
        Some(&self.atlas?.bind_group())
    }

    pub fn texture_ids(&self) -> &HashMap<TextureID, Texture2D> {
        &self.textures
    }

    pub fn vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }

    pub fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.index_buffer.as_ref()
    }

    pub fn entity_buffer(&self) -> Option<&wgpu::Buffer> {
        self.entity_buffer.as_ref()
    }

    pub fn index_count(&self) -> usize {
        self.entity_count * 6
    }

    pub fn entity_count(&self) -> usize {
        self.entity_count
    }
}

pub struct Layer2DSystem;

impl Layer2DSystem {
    fn create_entity_buffer(entities: &Vec<&Entity2D>, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Entity Buffer"),
            contents: bytemuck::cast_slice(
                entities
                    .iter()
                    .map(|e| e.to_raw())
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    fn create_index_buffer(device: &wgpu::Device, entity_count: usize) -> wgpu::Buffer {
        let mut indices = Vec::new();
        for _ in 0..entity_count {
            indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
        }
        indices.reserve(std::mem::size_of::<u16>() * entity_count as usize);
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(indices.as_slice()),
            usage: wgpu::BufferUsages::INDEX,
        })
    }

    fn create_vertex_buffer(entities: &Vec<&Entity2D>, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(
                entities
                    .iter()
                    .flat_map(|e| e.vertices())
                    .copied()
                    .collect::<Vec<_>>()
                    .as_slice(),
            ),
            usage: wgpu::BufferUsages::VERTEX,
        })
    }

    /// Update transformation data (not the vertices).
    // Panics of uninitialised
    pub fn update_entities(layer: &mut Layer2D, entities: Vec<&Entity2D>, queue: &wgpu::Queue) {
        if entities.len() > layer.entity_count() {
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
    pub fn set_entities(layer: &mut Layer2D, entities: Vec<&Entity2D>, device: &wgpu::Device) {
        // allocating exactly amount needed each time may increase the number of allocations needed..
        // perhaps a strategy of allocatin 2X needed data would be better
        layer.entity_count = entities.len();
        layer.entity_buffer = Some(Layer2DSystem::create_entity_buffer(&entities, device));
        layer.vertex_buffer = Some(Layer2DSystem::create_vertex_buffer(&entities, device));
        layer.index_buffer = Some(Layer2DSystem::create_index_buffer(
            device,
            layer.entity_count(),
        ));
    }

    // Same as set entities, but reuse the buffers, for when the number of entities hasn't grown
    // Panics if unintialised
    pub fn set_entities_fast(layer: &mut Layer2D, entities: Vec<&Entity2D>, queue: &wgpu::Queue) {
        if entities.len() > layer.entity_count() {
            panic!("Entities would not fit buffer")
        }
        layer.entity_count = entities.len();
        let data: Vec<Entity2DRaw> = entities.iter().map(|e| e.to_raw()).collect();
        // possibly extra copying going on here...look into it
        let vertices: Vec<Vertex> = entities.iter().flat_map(|e| *e.vertices()).collect();
        queue.write_buffer(
            &layer.entity_buffer.unwrap(),
            0,
            bytemuck::cast_slice(data.as_slice()),
        );
        queue.write_buffer(
            &layer.vertex_buffer.unwrap(),
            0,
            bytemuck::cast_slice(vertices.as_slice()),
        );
    }
}
