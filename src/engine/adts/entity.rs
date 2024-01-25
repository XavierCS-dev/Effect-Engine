use crate::engine::{
    primitives::{vector::Vector3, vertex::Vertex},
    texture::texture2d::Texture2D,
};

use super::{layer::LayerID, transform::Transform2D};

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
    vertices: Vec<Vertex>,
    // the actual numbers when in the renderer will vary depending on where the vertices are put in the main vertex buffer.
    // ie (index + num_of_vertices_in_buffer)
    // for now, write to buffer every frame, we can fix that later
    indices: Vec<u16>,
    transform: Transform2D,
}

impl Entity2D {
    pub fn new(layer: LayerID, texture: &str) -> Self {
        todo!();
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

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }

    pub fn indicies(&self) -> &Vec<u16> {
        &self.indices
    }
}
