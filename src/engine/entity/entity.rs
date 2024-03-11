use anyhow::Result;
use winit::dpi::PhysicalSize;

use crate::engine::{
    layer::layer::{Layer2D, LayerID},
    primitives::vector::Vector3,
    texture::texture2d::TextureID,
    util::effect_error::EffectError,
};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Entity2DRaw {
    position: [f32; 3],
    texture_index: [f32; 2],
    texture_size: [f32; 2],
}

impl Entity2DRaw {
    const ATTRIBUTE_ARRAY: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![2 => Float32x3, 3=> Float32x2, 4=>Float32x2];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Entity2DRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTE_ARRAY,
        }
    }
}

pub struct Entity2D {
    layer: LayerID,
    position: Vector3,
    texture: TextureID,
    texture_index: [u32; 2],
    texture_size: PhysicalSize<f32>,
}

impl Entity2D {
    pub fn new(position: Vector3, layer: &Layer2D, texture: TextureID) -> Self {
        let tex = layer.get_texture(texture).unwrap();
        let texture_index = tex.index().expect("Tex not in given layer");
        let texture_size = layer.tex_coord_size();
        Self {
            layer: layer.id(),
            position,
            texture,
            texture_index,
            texture_size,
        }
    }

    // will include "model" with pos and rotation later...
    pub fn to_raw(&self) -> Entity2DRaw {
        let position = [self.position.x, self.position.y, self.position.z];
        Entity2DRaw {
            position,
            texture_index: [self.texture_index[0] as f32, self.texture_index[1] as f32],
            texture_size: self.texture_size.into(),
        }
    }

    pub fn layer_id(&self) -> &LayerID {
        &self.layer
    }

    pub fn position(&self) -> &Vector3 {
        &self.position
    }
}

pub struct EntitySystem2D;

impl EntitySystem2D {
    pub fn set_texture(entity: &mut Entity2D, texture: TextureID, layer: &Layer2D) -> Result<()> {
        // find texture in layer, return the ID and set here, otherwise error
        let tex = layer
            .get_texture(texture)
            .ok_or(EffectError::new("Texture is not in given layer"))?;
        entity.texture_index = tex.index().unwrap();
        entity.texture_size = layer.tex_coord_size();
        entity.layer = layer.id();
        entity.texture = texture;
        Ok(())
    }
}
