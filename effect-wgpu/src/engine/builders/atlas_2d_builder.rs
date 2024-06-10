use winit::dpi::PhysicalSize;

use crate::texture::texture2d::{Texture2D, Texture2DBGL};

pub struct TextureAtlas2DBuilder {
    textures: Vec<Texture2D>,
    bind_group_layout_descriptor: wgpu::BindGroupLayoutDescriptor<'static>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    texture_size: PhysicalSize<u32>,
    pixel_art: bool,
}

impl Default for TextureAtlas2DBuilder {
    fn default() -> Self {
        let textures = Vec::new();
        let bind_group_layout_descriptor = Texture2D::layout();
        let bind_group_layout = None;
        let texture_size = PhysicalSize::new(0, 0);
        let pixel_art = true;
        Self {
            textures,
            bind_group_layout_descriptor,
            bind_group_layout,
            texture_size,
            pixel_art,
        }
    }
}

// Add init function for bind group layout.
// Panic if size hasn't been changed from 0.
impl TextureAtlas2DBuilder {}
