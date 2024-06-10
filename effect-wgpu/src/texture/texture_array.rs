use effect_util::effect_error::EffectError;
use winit::dpi::PhysicalSize;

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
    pub fn new(
        device: wgpu::Device,
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
        for (index, texture) in textures.iter_mut().enumerate() {
            let raw = image::open(texture.file_path())?;
            if PhysicalSize::new(texture.width(), texture.height) != texture_dimensions {
                bail!(EffectError::new(
                    format!(
                        "Expected texture dimensions: {:?}, found {:?} at index: {}",
                        texture_dimensions,
                        PhysicalSize::new(raw.width(), raw.height()),
                        index
                    )
                    .as_str(),
                ));
            }
        }

        todo!()
    }
}

// TODO: This needs to be changed to include the texture array, the index array and sampler array
impl TextureArray {
    fn layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
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
            ],
            label: Some("Bind group layout"),
        }
    }
}
