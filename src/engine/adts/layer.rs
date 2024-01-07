use anyhow::Result;
use std::collections::HashMap;

use crate::{
    engine::{
        primitives::vertex::Vertex,
        texture::{
            texture2d::{Texture2D, TextureID},
            texture_atlas2d::TextureAtlas2D,
            texture_pool::{BindGroupID, TexturePool2D},
        },
        traits::layer::Layer,
    },
    util::effect_error::EffectError,
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug)]
pub struct LayerID(pub u32);

pub struct Layer2D {
    id: LayerID,
    textures: HashMap<TextureID, Texture2D>,
    atlas: TextureAtlas2D,
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    entity_buffer: wgpu::Buffer,
}

impl Layer2D {
    pub fn new(
        id: LayerID,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Self> {
        let mut textures = HashMap::new();
        let atlas = TextureAtlas2D::new(texture.clone(), device, queue);
        textures.insert(texture.id().clone(), texture);
        Ok(Self {
            id,
            textures,
            atlas,
        })
    }

    pub fn add_texture(
        &mut self,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        Err(anyhow::Error::new(EffectError {
            msg: String::from("unimplemented"),
        }))
    }
}

impl Layer for Layer2D {
    fn bind_group(&self) -> &wgpu::BindGroup {
        self.atlas.bind_group()
    }

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.atlas.bind_group_layout()
    }

    fn texture_ids(&self) -> &HashMap<TextureID, Texture2D> {
        &self.textures
    }

    fn id(&self) -> LayerID {
        self.id
    }
}
