use crate::engine::{
    texture::texture2d::Texture2DSystem,
    util::{effect_error::EffectError, file_to_bytes::file_to_bytes},
};

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};

use super::texture2d::{Texture2D, TextureID};

use anyhow::{bail, Result};
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
    // remember 8196 limits
    pub fn new(
        textures: &mut Vec<Texture2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<Self> {
        let mut image_buffers: Vec<ImageBuffer<Rgba<u8>, _>> = Vec::new();
        let mut current_width = 0;
        let mut current_height = 0;
        let mut total_height = 0;
        for texture in textures.iter_mut() {
            let tex = image::open(texture.file_path())?;
            let tex_rgba = tex.to_rgba8();
            let tex_dimensions = tex.dimensions();
            Texture2DSystem::set_dimensions(texture, tex_dimensions.0, tex_dimensions.1);
            let pot_width = current_width + texture.width();
            let pot_depth = current_height + texture.height();
            // start from 0,0, place each image along the width until it would exceed max width.
            // Then start placing images along the height.
            if pot_width < MAX_WIDTH {
                if pot_depth < MAX_HEIGHT {
                    Texture2DSystem::set_offset(texture, current_width, current_height);
                    current_width = pot_width;
                    if pot_depth > total_height {
                        total_height = pot_depth;
                    }
                } else {
                    bail!(EffectError::new("Textures exceed atlas size"))
                }
            } else {
                let pot_depth = total_height + texture.height();
                let pot_width = texture.width();
                if pot_depth < MAX_HEIGHT {
                    Texture2DSystem::set_offset(texture, 0, total_height);
                    current_width = pot_width;
                    current_height = total_height;
                    total_height = pot_depth;
                } else {
                    bail!(EffectError::new("Textures exceed atlas size"))
                }
            }
            image_buffers.push(tex_rgba);
        }

        let mut combined_tex = ImageBuffer::new(current_width, total_height);

        for (tex_rgba, texture) in image_buffers.iter().zip(textures.iter_mut()) {
            let x = texture.offset().unwrap()[0];
            let y = texture.offset().unwrap()[1];
            combined_tex.copy_from(tex_rgba, x, y)?;
        }
        let extent = wgpu::Extent3d {
            width: combined_tex.width(),
            height: combined_tex.height(),
            depth_or_array_layers: 1,
        };
        let (bind_group, atlas, view, sampler) =
            Texture2DSystem::init_texture(extent, combined_tex, bind_group_layout, device, queue);
        Ok(Self {
            atlas,
            bind_group,
            view,
            sampler,
        })
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
