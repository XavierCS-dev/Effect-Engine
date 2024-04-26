use anyhow::Result;
use effect_core::{
    id::{LayerID, TextureID},
    primitives::vector::Vector3,
    raw::entityraw::Entity2DRaw,
    transform::{Transform2D, Transform2DSystem},
};
use effect_util::effect_error::EffectError;
use winit::dpi::PhysicalSize;

use crate::layer::WebLayer2D;

pub trait WebEntity2DRaw {
    const ATTRIBUTE_ARRAY: [wgpu::VertexAttribute; 6];
    fn layout() -> wgpu::VertexBufferLayout<'static>;
}

impl WebEntity2DRaw for Entity2DRaw {
    const ATTRIBUTE_ARRAY: [wgpu::VertexAttribute; 6] = wgpu::vertex_attr_array![2 => Float32x4, 3=> Float32x4,
        4=> Float32x4,5=> Float32x4,6=> Float32x2, 7=>Float32x2];

    fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Entity2DRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBUTE_ARRAY,
        }
    }
}

pub struct WebEntity2D {
    layer: LayerID,
    transform: Transform2D,
    texture: TextureID,
    texture_index: [u32; 2],
    texture_size: PhysicalSize<f32>,
}

impl WebEntity2D {
    pub fn new(position: Vector3<f32>, layer: &WebLayer2D, texture: TextureID) -> Self {
        let tex = layer.get_texture(texture).unwrap();
        let texture_index = tex.index().expect("Tex not in given layer");
        let texture_size = layer.tex_coord_size();
        let mut transform = Transform2D::new();
        Transform2DSystem::translate(&mut transform, position);
        Self {
            layer: layer.id(),
            transform,
            texture,
            texture_index,
            texture_size,
        }
    }

    pub fn to_raw(&self) -> Entity2DRaw {
        Entity2DRaw {
            transform: self.transform.to_raw().inner,
            texture_index: [self.texture_index[0] as f32, self.texture_index[1] as f32],
            texture_size: self.texture_size.into(),
        }
    }

    pub fn layer_id(&self) -> &LayerID {
        &self.layer
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.transform.position()
    }
}

pub struct EntitySystem2D;

impl EntitySystem2D {
    /// Sets tht texture of the given entity. The texture must be in the layer provided.
    /// Make sure to store a reference to this entity in the correct layer if you change it
    pub fn set_texture(
        entity: &mut WebEntity2D,
        texture: TextureID,
        layer: &WebLayer2D,
    ) -> Result<()> {
        let tex = layer
            .get_texture(texture)
            .ok_or(EffectError::new("Texture is not in given layer"))?;
        entity.texture_index = tex.index().unwrap();
        entity.texture_size = layer.tex_coord_size();
        entity.layer = layer.id();
        entity.texture = texture;
        Ok(())
    }

    pub fn set_position(entity: &mut WebEntity2D, position: Vector3<f32>) {
        Transform2DSystem::translate(&mut entity.transform, position);
    }
    pub fn set_rotation(entity: &mut WebEntity2D, degrees: f32) {
        Transform2DSystem::rotate(&mut entity.transform, degrees);
    }
    pub fn set_scale(entity: &mut WebEntity2D, scale: f32) {
        Transform2DSystem::scale(&mut entity.transform, scale);
    }
}
