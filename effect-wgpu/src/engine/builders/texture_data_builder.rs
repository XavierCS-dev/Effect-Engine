use crate::texture::{texture2d::Texture2D, texture_data::TextureData};
use anyhow::Result;
use winit::dpi::PhysicalSize;

// using a builder so expanding this with new options in the future is easy
pub struct TextureDataBuilder {
    dimensions: PhysicalSize<u32>,
    texture: Option<Texture2D>,
    pixel_art: bool,
}

impl Default for TextureDataBuilder {
    fn default() -> Self {
        let dimensions = PhysicalSize::new(0, 0);
        let texture = None;
        let pixel_art = false;

        Self {
            dimensions,
            texture,
            pixel_art,
        }
    }
}

impl TextureDataBuilder {
    pub fn dimensions(mut self, dimensions: PhysicalSize<u32>) -> Self {
        self.dimensions = dimensions;
        self
    }

    pub fn texture(mut self, texture: Texture2D) -> Self {
        self.texture = Some(texture);
        self
    }

    pub fn pixel_art(mut self, pixel_art: bool) -> Self {
        self.pixel_art = pixel_art;
        self
    }
}

// Texture pool will contain size data...as for IDs, pool can also keep a set of those, paired with the corresponding
// view, sampler, and index
impl TextureDataBuilder {
    pub fn build(mut self, device: &wgpu::Device, queue: &wgpu::Queue) -> Result<TextureData> {
        let texture = self
            .texture
            .expect("A texture must be supplied to the data builder");
        let tex = image::open(texture.file_path())?;
        let rgba_image = tex.to_rgba8();
        tex.resize(
            self.dimensions.width,
            self.dimensions.height,
            image::imageops::FilterType::Lanczos3,
        );
        let tex_rgba = tex.to_rgba8();
        let extent = wgpu::Extent3d {
            width: self.dimensions.width,
            height: self.dimensions.height,
            depth_or_array_layers: 1,
        };
        let wgpu_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("texture"),
            size: extent,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &wgpu_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba_image,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(extent.width * std::mem::size_of::<u32>() as u32),
                rows_per_image: Some(extent.height),
            },
            extent,
        );

        let view = wgpu_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mag_filter;
        if self.pixel_art {
            mag_filter = wgpu::FilterMode::Nearest;
        } else {
            mag_filter = wgpu::FilterMode::Linear
        }
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(TextureData {
            ID: texture.id,
            sampler,
            view,
        })
    }
}
