use std::num::NonZeroU64;

use effect_util::effect_error::EffectError;
use winit::dpi::PhysicalSize;

use crate::{
    engine::builders::texture_data_builder::TextureDataBuilder, texture::texture_data::TextureData,
};

use super::texture2d::Texture2D;

use anyhow::{bail, Result};

const MAX_TEXTURE_ARRAY_SIZE: usize = 256;

/*
Will need:
- Texture Data
- Texture indices
- Texture dimensions
- Texture count?
 */
// Will be uniform
pub struct TextureArray {
    texture_size: PhysicalSize<u32>,
    bind_group: wgpu::BindGroup,
}

impl TextureArray {
    // NEED TO FIGURE OUT HOW TO PASS PIXEL ART OPTION ON PER TEXTURE BASIS,
    // PERHAPS ANOTHER FIELD ON TEXTURE2D
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        textures: &mut Vec<Texture2D>,
        texture_dimensions: PhysicalSize<u32>,
    ) -> Result<Self> {
        if textures.len() > MAX_TEXTURE_ARRAY_SIZE {
            bail!(EffectError::new(
                format!(
                    "Expected maximum texture count of 256, found: {}",
                    textures.len()
                )
                .as_str(),
            ));
        }
        let tex_data: Vec<TextureData> = Vec::new();
        for (index, texture) in textures.iter_mut().enumerate() {
            // We don't need to check the size as it will be resized when using the builder, or will fail.
            tex_data.push(
                TextureDataBuilder::default()
                    .dimensions(texture_dimensions)
                    .texture(texture.clone())
                    .pixel_art(true)
                    .build(device, queue)
                    .expect(format!("Failed to create texture data, index: {}", index).as_str()),
            );
        }

        // Can't put this in its own function due to lifetime issues
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: true,
                        min_binding_size: Some(NonZeroU64::new(4).unwrap()),
                    },
                    count: None,
                },
            ],
            label: None,
        });
        let views: Vec<&wgpu::TextureView> = tex_data.iter().map(|f| &f.view).collect();
        let samplers: Vec<&wgpu::Sampler> = tex_data.iter().map(|f| &f.sampler).collect();
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(&views),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::SamplerArray(&samplers),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        // figure out how texture indices will be deduced....
                        buffer: &texture_index_buffer,
                        offset: 0,
                        size: Some(NonZeroU64::new(4).unwrap()),
                    }),
                },
            ],
            layout: &bgl,
            label: Some("bind group"),
        });
        todo!()
    }
}