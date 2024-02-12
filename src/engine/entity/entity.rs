use crate::engine::{
    layer::layer::{Layer2D, LayerID},
    primitives::{vector::Vector3, vertex::Vertex},
    texture::{
        texture2d::Texture2D,
        texture_atlas2d::TextureAtlas2D,
        texture_pool::{self, TexturePool2D},
    },
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
        wgpu::vertex_attr_array![2 => Float32x3, 3=> Float32x2];

    pub fn layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTE_ARRAY,
        }
    }
}

pub struct Entity2D {
    layer: LayerID,
    position: Vector3,
    texture: Texture2D,
    vertex_group: VertexGroup2D,
}

impl Entity2D {
    pub fn new(
        position: Vector3,
        layer: LayerID,
        texture: Texture2D,
        screen_width: u32,
        screen_height: u32,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let vertex_group = VertexGroup2D::new(&texture, screen_width, screen_height);
        Self {
            layer,
            position,
            texture,
            vertex_group,
        }
    }

    // will include "model" with pos and rotation later...
    pub fn to_raw(&self) -> Entity2DRaw {
        let position = [self.position.x, self.position.y, self.position.z];
        let texture_offset = self.texture.offset().expect("Texture is uninitiliased");
        Entity2DRaw {
            position,
            texture_offset,
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