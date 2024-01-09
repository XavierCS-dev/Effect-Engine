use std::collections::HashMap;

use crate::engine::{
    primitives::{vector::Vector3D, vertex::Vertex},
    texture::texture2d::Texture2D,
    traits::entity::EntityType,
};

use super::{layer::LayerID, transform::Transform2D};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Entity2DRaw {
    position: [f32; 3],
    texture_offset: [u32; 2],
}

pub struct Entity2D {
    layer: LayerID,
    position: Vector3D,
    texture: Texture2D,
    vertices: Vec<Vertex>,
    // the actual numbers when in the renderer will vary depending on where the vertices are put in the main vertex buffer.
    // ie (index + num_of_vertices_in_buffer)
    // for now, write to buffer every frame, we can fix that later
    indices: Vec<Vertex>,
    transform: Transform2D,
}

impl Entity2D {
    // will include "model" with pos and rotation later...
    fn to_raw(&self) -> Entity2DRaw {
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

    pub fn position(&self) -> &Vector3D {
        &self.position
    }
}
