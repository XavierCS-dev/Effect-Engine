use std::{collections::HashMap, num::NonZeroU64};

use effect_core::id::TextureID;
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
// Texture2Ds will be copyable, and non modifiable
// Texture2Ds can be requested from their respective layer
// User supplies path and ID, not a Texture2D
pub struct TextureArray {
    texture_dimensions: PhysicalSize<u32>,
    bind_group: wgpu::BindGroup,
    textures: HashMap<TextureID, Texture2D>,
}

impl TextureArray {
    // NEED TO FIGURE OUT HOW TO PASS PIXEL ART OPTION ON PER TEXTURE BASIS,
    // PERHAPS ANOTHER FIELD ON TEXTURE2D
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        textures: Vec<(TextureID, &'static str)>,
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
        let mut tex_data: Vec<TextureData> = Vec::new();
        for (index, texture) in textures.iter().enumerate() {
            // We don't need to check the size as it will be resized when using the builder, or will fail.
            tex_data.push(
                TextureDataBuilder::default()
                    .dimensions(texture_dimensions)
                    .path(texture.1)
                    .pixel_art(true)
                    .build(device, queue)
                    .expect(format!("Failed to create texture data, index: {}", index).as_str()),
            );
        }

        let textures = textures
            .iter()
            .enumerate()
            .map(|(index, (id, path))| (*id, Texture2D::new(*id, *path, index)))
            .collect();

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

        // Texture index and ID will be stored here..each ID corresponds with an index. Should be fetched only when switching
        // to a new texture, and not every time
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
            ],
            layout: &bgl,
            label: Some("bind group"),
        });
        let texture_size = texture_dimensions;
        Ok(Self {
            textures,
            texture_dimensions,
            bind_group,
        })
    }

    pub fn texture(&self, id: &TextureID) -> Option<Texture2D> {
        self.textures.get(id).copied()
    }

    pub fn textures(&self) -> &HashMap<TextureID, Texture2D> {
        &self.textures
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn texture_dimensions(&self) -> PhysicalSize<u32> {
        self.texture_dimensions
    }
}
