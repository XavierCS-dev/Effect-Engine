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
    traits::layer::Layer,
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct LayerID(pub u32);
pub struct Initialised;
pub struct Unitialised;

pub struct Layer2D<State = Unitialised> {
    id: LayerID,
    textures: HashMap<TextureID, Texture2D>,
    atlas: Option<TextureAtlas2D>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: wgpu::Buffer,
    entity_count: u32,
    entity_buffer: Option<wgpu::Buffer>,
    state: std::marker::PhantomData<State>,
}

impl Layer2D {
    pub fn new(id: LayerID, texture: Texture2D, device: &wgpu::Device) -> Result<Self> {
        let mut textures = HashMap::new();
        let index_buffer = Layer2DSystem::create_index_buffer(device);
        let entity_count = 0;
        let state = std::marker::PhantomData::default();
        Ok(Self {
            id,
            textures,
            atlas: None,
            vertex_buffer: None,
            index_buffer,
            entity_count,
            entity_buffer: None,
            state,
        })
    }

    pub fn id(&self) -> LayerID {
        self.id
    }

    pub fn contains_texture(&self, texture_id: &TextureID) -> bool {
        self.textures.contains_key(texture_id)
    }
}

impl Layer for Layer2D {
    fn bind_group(&self) -> Option<&wgpu::BindGroup> {
        Some(&self.atlas?.bind_group())
    }

    fn texture_ids(&self) -> &HashMap<TextureID, Texture2D> {
        &self.textures
    }

    fn vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }

    fn index_buffer(&self) -> &wgpu::Buffer {
        &self.index_buffer
    }

    fn entity_buffer(&self) -> Option<&wgpu::Buffer> {
        self.entity_buffer.as_ref()
    }

    fn index_count(&self) -> usize {
        (self.entity_count * 6) as usize
    }

    fn id(&self) -> LayerID {
        self.id
    }
}

pub struct Layer2DSystem;

impl Layer2DSystem {
    fn create_entity_buffer(entities: &Vec<&Entity2D>, device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
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

    fn create_index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&([0, 1, 2, 0, 2, 3] as [u16; 6])),
            usage: wgpu::BufferUsages::VERTEX,
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

    pub fn add_texture(
        layer: &mut Layer2D,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<()> {
        match layer
            .atlas
            .unwrap()
            .add_texture(texture.clone(), device, queue, bind_group_layout)
        {
            Ok(_) => {
                layer.textures.insert(texture.id().to_owned(), texture);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Update transformation data (not the vertices).
    pub fn update_entities(
        layer: &mut Layer2D<Initialised>,
        entities: Vec<&Entity2D>,
        queue: &wgpu::Queue,
    ) {
        if entities.len() as u32 > layer.entity_count {
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
        layer.entity_count = entities.len() as u32;
        let data: Vec<Entity2DRaw> = entities.iter().map(|e| e.to_raw()).collect();
        // possibly extra copying going on here...look into it
        let vertices: Vec<Vertex> = entities.iter().flat_map(|e| *e.vertices()).collect();
        layer.entity_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Entity Buffer"),
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
        layer.vertex_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
    }

    // Same as set entities, but reuse the buffers, for when the number of entities hasn't grown
    pub fn set_entities_fast(
        layer: &mut Layer2D<Initialised>,
        entities: Vec<&Entity2D>,
        queue: &wgpu::Queue,
    ) {
        if entities.len() as u32 > layer.entity_count {
            panic!("Entities would not fit buffer")
        }
        let data: Vec<Entity2DRaw> = entities.iter().map(|e| e.to_raw()).collect();
        // possibly extra copying going on here...look into it
        let vertices: Vec<Vertex> = entities.iter().flat_map(|e| *e.vertices()).collect();
        queue.write_buffer(
            &layer.entity_buffer.unwrap(),
            0,
            bytemuck::cast_slice(&data),
        );
        queue.write_buffer(
            &layer.vertex_buffer.unwrap(),
            0,
            bytemuck::cast_slice(&vertices),
        );
    }
}
