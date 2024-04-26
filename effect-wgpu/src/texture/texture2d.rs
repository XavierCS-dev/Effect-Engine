use effect_core::id::TextureID;
use image::ImageBuffer;
use image::Rgba;

#[derive(Clone, Debug)]
pub struct WebTexture2D {
    pub id: TextureID,
    pub path: &'static str,
    pub width: u32,
    pub height: u32,
    pub index: Option<[u32; 2]>,
}

impl WebTexture2D {
    pub fn new(id: TextureID, path: &'static str) -> Self {
        Self {
            id,
            path,
            width: 0,
            height: 0,
            index: None,
        }
    }

    pub fn file_path(&self) -> &str {
        self.path
    }

    pub fn id(&self) -> &TextureID {
        &self.id
    }

    pub fn index(&self) -> Option<[u32; 2]> {
        self.index
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub struct WebTexture2DSystem;
impl WebTexture2DSystem {
    pub fn set_index(texture: &mut WebTexture2D, index: [u32; 2]) {
        texture.index = Some(index);
    }

    pub fn set_dimensions(texture: &mut WebTexture2D, width: u32, height: u32) {
        texture.width = width;
        texture.height = height;
    }

    pub fn init_texture(
        extent: wgpu::Extent3d,
        rgba_image: ImageBuffer<Rgba<u8>, Vec<u8>>,
        bind_group_layout: &wgpu::BindGroupLayout,
        pixel_art: bool,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::BindGroup {
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
        let mag_filter;
        if pixel_art {
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

        bind_group
    }
}
