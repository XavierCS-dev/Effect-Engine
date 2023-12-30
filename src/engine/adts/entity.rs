use std::collections::HashMap;

use crate::engine::{
    primitives::{vector::Vector2D, vertex::Vertex},
    texture::texture2d::Texture2D,
    traits::entity::EntityType,
};

use super::transform::Transform2D;

pub struct Entity2DRaw {}

pub struct Entity2D {
    position: Vector2D,
    texture: Texture2D,
    vertices: Vec<Vertex>,
    transform: Transform2D,
}

impl EntityType for Entity2D {
    fn to_raw(&self) -> Entity2DRaw {
        Entity2DRaw {}
    }
}
