use std::collections::HashMap;

use crate::engine::{
    primitives::{vector::Vector3D, vertex::Vertex},
    texture::texture2d::Texture2D,
    traits::entity::EntityType,
};

use super::transform::Transform2D;

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct Entity2DRaw {
    position: [f32; 3],
    texture_offset: [u32; 2],
}

pub struct Entity2D {
    position: Vector3D,
    texture: Texture2D,
    vertices: Vec<Vertex>,
    transform: Transform2D,
}

impl EntityType for Entity2D {
    fn to_raw(&self) -> Entity2DRaw {
        let position = [self.position.x, self.position.y, self.position.z];
        let texture_offset = self.texture.offset().expect("Texture is uninitiliased");
        Entity2DRaw {
            position,
            texture_offset,
        }
    }
}
