use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;
use std::fs;
use std::io::prelude::*;

use super::texture_pool::BindGroupID;

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Debug)]
pub struct TextureID(pub String);

impl TextureID {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// add tex coords here, make bind group mandatory.
#[derive(Clone)]
pub struct Texture2D {
    id: TextureID,
    path: &'static str,
    width: u32,
    height: u32,
    offset: Option<[u32; 2]>,
}

impl Texture2D {
    pub fn new(id: TextureID, path: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        // havig one bind group per texure isn't very performant.
        // It may be better to have one bind group per zone of loaded textures,
        // each bind group having all the textures it needs for a zone,
        // then swap out the bind group for new zones.
        Self {
            id,
            path,
            width: 0,
            height: 0,
            offset: None,
        }
    }

    pub fn file_path(&self) -> &str {
        self.path
    }

    pub fn init_texture(
        extent: wgpu::Extent3d,
        rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
        bind_group_layout: &wgpu::BindGroupLayout,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> (
        wgpu::BindGroup,
        wgpu::Texture,
        wgpu::TextureView,
        wgpu::Sampler,
    ) {
        let texture = device.create_texture(&wgpu::TextureDescriptor {
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
                texture: &texture,
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

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind group"),
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        (bind_group, texture, view, sampler)
    }

    pub fn id(&self) -> &TextureID {
        &self.id
    }

    pub fn offset(&self) -> Option<[u32; 2]> {
        self.offset
    }

    pub fn set_offset(&mut self, x: u32, y: u32) {
        self.offset = Some([x, y]);
    }

    pub fn set_dimensions(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
