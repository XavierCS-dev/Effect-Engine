use winit::dpi::PhysicalSize;

use crate::texture::texture2d::{WebTexture2D, WebTexture2DBGL};

pub struct WebTextureAtlas2DBuilder {
    textures: Vec<WebTexture2D>,
    bind_group_layout_descriptor: wgpu::BindGroupLayoutDescriptor<'static>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    texture_size: PhysicalSize<u32>,
    pixel_art: bool,
}

impl Default for WebTextureAtlas2DBuilder {
    fn default() -> Self {
        let textures = Vec::new();
        let bind_group_layout_descriptor = WebTexture2D::layout();
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
impl WebTextureAtlas2DBuilder {}
