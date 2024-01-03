use anyhow::Result;
use std::collections::HashMap;

use crate::engine::{
    texture::{
        texture2d::{Texture2D, TextureID},
        texture_atlas2d::TextureAtlas2D,
        texture_pool::{BindGroupID, TexturePool2D},
    },
    traits::layer::Layer,
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug)]
pub struct LayerID(pub u32);

pub struct Layer2D {
    id: LayerID,
    textures: HashMap<TextureID, Texture2D>,
    atlas: TextureAtlas2D,
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
}

impl Layer for Layer2D {
    fn bind_group(&self) -> &wgpu::BindGroup {
        self.atlas.bind_group()
    }

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        self.atlas.bind_group_layout()
    }

    fn id(&self) -> LayerID {
        self.id
    }
}
