use anyhow::Result;
use std::collections::HashMap;

use wgpu::util::DeviceExt;

use crate::engine::{
    entity::entity::Entity2D,
    texture::{
        texture2d::{Texture2D, TextureID},
        texture_atlas2d::TextureAtlas2D,
    },
    traits::layer::Layer,
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct LayerID(pub u32);

pub struct Layer2D {
    id: LayerID,
    textures: HashMap<TextureID, Texture2D>,
    atlas: TextureAtlas2D,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    entity_buffer: Option<wgpu::Buffer>,
    entity_count: usize,
    indices: usize,
}

impl Layer2D {
    pub fn new(
        id: LayerID,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        let mut textures = HashMap::new();
        let atlas = TextureAtlas2D::new(texture.clone(), device, queue, bind_group_layout);
        textures.insert(texture.id().clone(), texture);
        let entity_count = 0;
        Ok(Self {
            id,
            textures,
            atlas,
            vertex_buffer: None,
            index_buffer: None,
            entity_buffer: None,
            entity_count,
            indices: 0,
        })
    }

    fn create_entity_buffer(&mut self, entities: &Vec<&Entity2D>, device: &wgpu::Device) {
        self.entity_buffer = Some(
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
            }),
        );
    }

    fn create_index_buffer(&mut self, entities: &Vec<&Entity2D>, device: &wgpu::Device) {
        self.index_buffer = Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(
                    entities
                        .iter()
                        .flat_map(|e| e.indicies())
                        .copied()
                        .collect::<Vec<_>>()
                        .as_slice(),
                ),
                usage: wgpu::BufferUsages::VERTEX,
            }),
        );
    }

    fn create_vertex_buffer(&mut self, entities: &Vec<&Entity2D>, device: &wgpu::Device) {
        self.vertex_buffer = Some(
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
            }),
        );
    }

    pub fn add_texture(
        &mut self,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<()> {
        match self
            .atlas
            .add_texture(texture.clone(), device, queue, bind_group_layout)
        {
            Ok(_) => {
                self.textures.insert(texture.id().to_owned(), texture);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }
}

impl Layer for Layer2D {
    fn bind_group(&self) -> &wgpu::BindGroup {
        self.atlas.bind_group()
    }

    fn texture_ids(&self) -> &HashMap<TextureID, Texture2D> {
        &self.textures
    }

    fn vertex_buffer(&self) -> Option<&wgpu::Buffer> {
        self.vertex_buffer.as_ref()
    }

    fn index_buffer(&self) -> Option<&wgpu::Buffer> {
        self.index_buffer.as_ref()
    }

    fn entity_buffer(&self) -> Option<&wgpu::Buffer> {
        self.entity_buffer.as_ref()
    }

    fn index_count(&self) -> usize {
        self.indices
    }

    fn set_vertex_buffers(
        &mut self,
        entities: &Vec<&Entity2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        match self.vertex_buffer() {
            Some(v_buf) => {
                if entities.len() > self.entity_count {
                    self.create_vertex_buffer(entities, device)
                } else {
                    queue.write_buffer(
                        self.vertex_buffer.as_ref().unwrap(),
                        0,
                        bytemuck::cast_slice(
                            entities
                                .iter()
                                .flat_map(|e| e.vertices())
                                .copied()
                                .collect::<Vec<_>>()
                                .as_slice(),
                        ),
                    )
                }
            }
            None => {
                self.create_vertex_buffer(entities, device);
            }
        };
        match self.index_buffer() {
            Some(i_buf) => {
                if entities.len() > self.entity_count {
                    self.create_vertex_buffer(entities, device)
                } else {
                    queue.write_buffer(
                        self.index_buffer.as_ref().unwrap(),
                        0,
                        bytemuck::cast_slice(
                            entities
                                .iter()
                                .flat_map(|e| e.indicies())
                                .copied()
                                .collect::<Vec<_>>()
                                .as_slice(),
                        ),
                    )
                }
            }
            None => {
                self.create_index_buffer(entities, device);
            }
        }
        // if the vertices have changed, the entities probably have to
        // Given this is a 2D engine tho, everything should be a quad...
        // meaning, it is pretty much guaranteed new entities were created
        self.set_entity_buffer(entities, device, queue, entities.len() > self.entity_count)?;
        self.entity_count = entities.len();
        self.indices = entities.iter().fold(0, |acc, e| acc + e.indicies().len());
        Ok(())
    }

    fn set_entity_buffer(
        &mut self,
        entities: &Vec<&Entity2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        recreate_buffer: bool,
    ) -> Result<()> {
        match self.entity_buffer() {
            Some(e_buf) => {
                if recreate_buffer {
                    self.create_entity_buffer(entities, device);
                } else {
                    queue.write_buffer(
                        self.entity_buffer.as_ref().unwrap(),
                        0,
                        bytemuck::cast_slice(
                            entities
                                .iter()
                                .map(|e| e.to_raw())
                                .collect::<Vec<_>>()
                                .as_slice(),
                        ),
                    )
                }
            }
            None => {
                self.create_entity_buffer(entities, device);
            }
        }
        Ok(())
    }

    fn id(&self) -> LayerID {
        self.id
    }
}
