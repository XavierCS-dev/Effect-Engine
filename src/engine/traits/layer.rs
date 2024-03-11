use std::collections::HashMap;

use anyhow::Result;

use crate::engine::{
    layer::layer::LayerID,
    texture::texture2d::{Texture2D, TextureID},
};

pub trait Layer {
    fn bind_group(&self) -> Option<&wgpu::BindGroup>;

    fn texture_ids(&self) -> &HashMap<TextureID, Texture2D>;

    fn vertex_buffer(&self) -> Option<&wgpu::Buffer>;

    fn index_buffer(&self) -> &wgpu::Buffer;

    fn entity_buffer(&self) -> Option<&wgpu::Buffer>;

    fn index_count(&self) -> usize;

    fn id(&self) -> LayerID;

    fn entity_count(&self) -> u32;
}
