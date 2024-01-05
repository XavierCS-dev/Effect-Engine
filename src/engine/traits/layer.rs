use std::collections::HashMap;

use crate::engine::{
    adts::layer::LayerID,
    texture::texture2d::{Texture2D, TextureID},
};

pub trait Layer {
    fn bind_group(&self) -> &wgpu::BindGroup;

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;

    fn texture_ids(&self) -> &HashMap<TextureID, Texture2D>;

    fn id(&self) -> LayerID;
}
