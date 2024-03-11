use crate::engine::{
    texture::texture2d::Texture2DSystem,
    util::{effect_error::EffectError, file_to_bytes::file_to_bytes},
};

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};
use winit::dpi::PhysicalSize;

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
    dimensions: PhysicalSize<u32>,
    tex_coord_size: PhysicalSize<f32>,
}

impl TextureAtlas2D {
    // SWITCH TO CREATION OF ATLAS THEN CAN'T BE MODIFIED
    // remember 8196 limits
    pub fn new(
        textures: &mut Vec<Texture2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        texture_size: PhysicalSize<u32>,
    ) -> Result<Self> {
        let width_count = MAX_WIDTH / texture_size.width;
        let height_count = MAX_HEIGHT / texture_size.height;
        if (width_count as usize * height_count as usize) < textures.len() {
            bail!(EffectError::new("Not enough space for textures in atlas"));
        }

        let extent = wgpu::Extent3d {
            width: 64,
            height: 32,
            depth_or_array_layers: 1,
        };

        let mut current_width = 0;
        let mut current_height = 0;
        let mut row = 0;
        let mut column = 0;
        let mut dimensions: Vec<(u32, u32)> = Vec::new();
        let mut image_buffers = Vec::new();
        for texture in textures.iter_mut() {
            let tex = image::open(texture.file_path())?;
            tex.resize(
                texture_size.width,
                texture_size.height,
                image::imageops::FilterType::Lanczos3,
            );
            let tex_rgba = tex.to_rgba8();
            Texture2DSystem::set_dimensions(texture, texture_size.width, texture_size.height);

            let pot_width = current_width + texture_size.width;
            let mut new_row = false;
            if pot_width > MAX_WIDTH {
                row += 1;
                column = 0;
                new_row = true;
            }
            Texture2DSystem::set_index(texture, [column, row]);

            image_buffers.push(tex_rgba);
            dimensions.push((current_width, current_height));

            current_width += texture_size.width;
            column += 1;
            if new_row {
                current_height += texture_size.height;
            }
        }

        let total_height = texture_size.height + (texture_size.height * row);
        let total_width;
        if width_count > textures.len() as u32 {
            total_width = current_width;
        } else {
            total_width = MAX_WIDTH - (MAX_WIDTH % width_count);
        }
        let mut combined_tex = ImageBuffer::new(total_width, total_height);
        for ((width, height), texture) in dimensions.iter().zip(image_buffers) {
            combined_tex.copy_from(&texture, *width, *height).unwrap();
        }

        image::save_buffer_with_format(
            "memes.png",
            &combined_tex,
            64,
            32,
            image::ColorType::Rgba8,
            image::ImageFormat::Png,
        )
        .unwrap();

        let (bind_group, atlas, view, sampler) =
            Texture2DSystem::init_texture(extent, combined_tex, bind_group_layout, device, queue);
        let dimensions = PhysicalSize::new(total_width, total_height);
        let tex_coord_width = (texture_size.width as f64 / total_width as f64) as f32;
        let tex_coord_height = (texture_size.height as f64 / total_height as f64) as f32;
        let tex_coord_size = PhysicalSize::new(tex_coord_width, tex_coord_height);
        Ok(Self {
            atlas,
            bind_group,
            view,
            sampler,
            dimensions,
            tex_coord_size,
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

    pub fn dimensions(&self) -> PhysicalSize<u32> {
        self.dimensions
    }

    pub fn tex_coord_size(&self) -> PhysicalSize<f32> {
        self.tex_coord_size
    }
}
