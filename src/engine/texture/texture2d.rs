use image::DynamicImage;
use image::GenericImageView;
use image::ImageBuffer;
use image::Rgba;
use std::fs;
use std::io;
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
    path: String,
    bind_group_id: Option<BindGroupID>,
    width: u32,
    height: u32,
    offset: Option<(u32, u32)>,
}

impl Texture2D {
    pub fn new(id: TextureID, filepath: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let bind_group_id = None;
        let mut file_byes: Vec<u8> = Vec::new();
        let mut file = fs::File::open(filepath).expect("Could not find file {filepath}");
        file.read_to_end(&mut file_byes).unwrap();
        let diffuse = image::load_from_memory(file_byes.as_slice()).unwrap();
        let diffuse_rgb = diffuse.to_rgba8();
        let dimensions = diffuse.dimensions();

        let texure_extent = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        // havig one bind group per texure isn't very performant.
        // It may be better to have one bind group per zone of loaded textures,
        // each bind group having all the textures it needs for a zone,
        // then swap out the bind group for new zones.
        Self {
            id,
            path: filepath.to_string(),
            bind_group_id,
            width: dimensions.0,
            height: dimensions.1,
            offset: None,
        }
    }

    pub fn file_path(&self) -> &String {
        &self.path
    }

    pub fn init_texture(
        extent: wgpu::Extent3d,
        dynamic_image: DynamicImage,
        rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> (
        wgpu::BindGroup,
        wgpu::BindGroupLayout,
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

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
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
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bind group"),
            layout: &bind_group_layout,
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

        (bind_group, bind_group_layout, texture, view, sampler)
    }

    pub fn id(&self) -> &TextureID {
        &self.id
    }

    pub fn path(&self) -> &str {
        self.path.as_str()
    }

    pub fn offset(&self) -> Option<(u32, u32)> {
        self.offset
    }

    pub fn set_offset(&mut self, x: u32, y: u32) {
        self.offset = Some((x, y));
    }

    pub fn bind_group_id(&self) -> Option<BindGroupID> {
        self.bind_group_id
    }

    pub fn set_bind_group_id(&mut self, id: BindGroupID) {
        self.bind_group_id = Some(id);
    }
}
