use crate::engine::util::{effect_error::EffectError, file_to_bytes::file_to_bytes};

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};

use super::texture2d::{Texture2D, TextureID};

use anyhow::Result;
use image::EncodableLayout;

const MAX_WIDTH: u32 = 8196;
const MAX_HEIGHT: u32 = 8196;

pub struct TextureAtlas2D {
    bind_group: wgpu::BindGroup,
    atlas: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl TextureAtlas2D {
    // SWITCH TO CREATION OF ATLAS THEN CAN'T BE MODIFIED
    pub fn new(
        textures: Vec<&mut Texture2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        todo!()
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn atlas(&self) -> &wgpu::Texture {
        &self.atlas
    }

    pub fn view(&self) -> &wgpu::TextureView {
        &self.view
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
