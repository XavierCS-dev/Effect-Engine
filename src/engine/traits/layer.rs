use std::collections::HashMap;

use anyhow::Result;

use crate::engine::{
    adts::{entity::Entity2D, layer::LayerID},
    texture::texture2d::{Texture2D, TextureID},
};

pub trait Layer {
    fn bind_group(&self) -> &wgpu::BindGroup;

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;

    fn texture_ids(&self) -> &HashMap<TextureID, Texture2D>;

    fn vertex_buffer(&self) -> Option<&wgpu::Buffer>;

    fn index_buffer(&self) -> Option<&wgpu::Buffer>;

    fn entity_buffer(&self) -> Option<&wgpu::Buffer>;

    fn index_count(&self) -> usize;

    fn set_vertex_buffers(
        &mut self,
        entities: &Vec<&Entity2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()>;

    fn set_entity_buffer(
        &mut self,
        entities: &Vec<&Entity2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        recreate_buffer: bool,
    ) -> Result<()>;

    fn id(&self) -> LayerID;
}
