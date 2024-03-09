use anyhow::Result;

use crate::engine::{
    layer::layer::{Layer2D, LayerID},
    primitives::{vector::Vector3, vertex::Vertex},
    texture::{
        texture2d::{Texture2D, TextureID},
        texture_atlas2d::TextureAtlas2D,
    },
    util::effect_error::EffectError,
};

use super::vertex_group::VertexGroup2D;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Entity2DRaw {
    position: [f32; 3],
    texture_offset: [u32; 2],
}

impl Entity2DRaw {
    const ATTRIBUTE_ARRAY: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![2 => Float32x3, 3=> Uint32x2];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Entity2DRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTE_ARRAY,
        }
    }

    // This will likely change, having a central place for this makes things way easier
    pub fn size() -> usize {
        use std::mem;
        mem::size_of::<[f32; 3]>() + mem::size_of::<[u32; 2]>()
    }
}

pub struct Entity2D {
    layer: LayerID,
    position: Vector3,
    texture: TextureID,
    vertex_group: VertexGroup2D,
    texture_offset: [u32; 2],
}

impl Entity2D {
    pub fn new(
        position: Vector3,
        layer: &Layer2D,
        texture: TextureID,
        screen_width: u32,
        screen_height: u32,
    ) -> Self {
        let tex = layer.get_texture(texture).unwrap();
        let vertex_group = VertexGroup2D::new(
            tex.width(),
            tex.height(),
            screen_width,
            screen_height,
            layer.atlas_dimensions(),
            tex.offset().unwrap(),
        );
        let layer = layer.id();
        let texture_offset = tex.offset().unwrap();
        Self {
            layer,
            position,
            texture,
            vertex_group,
            texture_offset,
        }
    }

    // will include "model" with pos and rotation later...
    pub fn to_raw(&self) -> Entity2DRaw {
        let position = [self.position.x, self.position.y, self.position.z];
        Entity2DRaw {
            position,
            texture_offset: self.texture_offset,
        }
    }

    pub fn layer_id(&self) -> &LayerID {
        &self.layer
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }

    pub fn vertices(&self) -> &[Vertex; 4] {
        self.vertex_group.vertices()
    }
}

pub struct EntitySystem2D;

impl EntitySystem2D {
    pub fn set_texture(entity: &mut Entity2D, texture: TextureID, layer: &Layer2D) -> Result<()> {
        // find texture in layer, return the ID and set here, otherwise error
        let tex = layer
            .get_texture(texture)
            .ok_or(EffectError::new("Texture is not in given layer"))?;
        entity.texture_offset = tex.offset().unwrap();
        entity.vertex_group = VertexGroup2D::new(
            tex.width(),
            tex.height(),
            layer.width(),
            layer.height(),
            layer.atlas_dimensions(),
            tex.offset().unwrap(),
        );
        entity.texture = texture;
        Ok(())
    }
}
