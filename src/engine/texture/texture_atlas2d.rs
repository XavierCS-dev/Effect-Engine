use crate::engine::util::{effect_error::EffectError, file_to_bytes::file_to_bytes};

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};

use super::texture2d::{Texture2D, TextureID};

use anyhow::Result;
use image::EncodableLayout;

const MAX_WIDTH: u32 = 8196;
const MAX_HEIGHT: u32 = 8196;

pub struct TextureAtlas2D {
    bind_group: wgpu::BindGroup,
    textures: Vec<Texture2D>,
    atlas: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl TextureAtlas2D {
    pub fn new(
        mut texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let mut textures = Vec::new();
        let file_bytes = file_to_bytes(texture.file_path().as_str());
        let image_bytes = image::load_from_memory(file_bytes.as_bytes())
            .expect(format!("Texture {} not found", texture.file_path()).as_str());
        let image_rgba = image_bytes.to_rgba8();
        let dimensions = image_bytes.dimensions();

        let texture_extent = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let (bind_group, atlas, view, sampler) = Texture2D::init_texture(
            texture_extent,
            image_rgba,
            bind_group_layout,
            &device,
            &queue,
        );
        texture.set_offset(0, 0);
        Self {
            textures,
            bind_group,
            atlas,
            view,
            sampler,
        }
    }

    pub fn add_texture(
        &mut self,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Result<()> {
        let mut image_buffers: Vec<ImageBuffer<Rgba<u8>, _>> = Vec::new();
        let mut total_width = 0;
        let mut total_height = 0;
        let mut current_width = 0;
        let mut current_height = 0;

        self.textures.push(texture.clone());

        for tex_local in &mut self.textures {
            let file_bytes = file_to_bytes(tex_local.file_path().as_str());
            let image_bytes = match image::load_from_memory(file_bytes.as_bytes()) {
                Ok(b) => b,
                Err(_) => {
                    // shadow var above, don't override..
                    let tex_local = self.textures.pop().unwrap();
                    return Err(anyhow::Error::new(EffectError::new(
                        format!("Texture {} not found", tex_local.file_path()).as_str(),
                    )));
                }
            };
            let image_rgba = image_bytes.to_rgba8();
            let dimensions = image_bytes.dimensions();

            if (current_width + dimensions.0) > MAX_WIDTH {
                // to not overwrite textures, the next texture will need to be placed at the highest point.
                // this is why we check total_height not current_height + dimensions.1
                if (total_height + dimensions.1) > MAX_HEIGHT {
                    self.textures.pop();
                    return Err(anyhow::Error::new(EffectError::new(
                        "Texture atlas would exceed max size, try a different layer",
                    )));
                }
                current_height = total_height;
                total_height += dimensions.1;
                current_width = 0;
            }

            // Make sure texture isn't too long to where it extends past the max
            if (current_height + dimensions.1) > MAX_HEIGHT {
                self.textures.pop();
                return Err(anyhow::Error::new(EffectError::new(
                    "Texture atlas would exceed max size, try a different layer",
                )));
            }

            tex_local.set_offset(current_width, current_height);
            if (current_width + dimensions.0) > total_width {
                total_width = current_width + dimensions.0;
            }
            if (current_height + dimensions.1) > total_height {
                total_height = current_height + dimensions.1;
            }

            image_buffers.push(image_rgba);
        }
        let mut combined_image = ImageBuffer::new(total_width, total_height);

        for (index, image_rgba) in image_buffers.iter().enumerate() {
            // if this fails, there is a mismatch between the image bytes and the textures in self.textures.
            // it is most likely related to the code which calculates the offsets.
            let dimensions = unsafe { self.textures.get_unchecked(index).offset().unwrap() };
            unsafe {
                combined_image
                    .copy_from(image_rgba, dimensions[0], dimensions[1])
                    .unwrap();
            }
        }
        let extent = wgpu::Extent3d {
            width: total_width,
            height: total_height,
            depth_or_array_layers: 1,
        };

        let (bind_group, atlas, _, _) =
            Texture2D::init_texture(extent, combined_image, bind_group_layout, device, queue);
        self.atlas = atlas;
        self.bind_group = bind_group;
        self.textures.push(texture);
        Ok(())
    }

    pub fn add_textures(
        &mut self,
        textures: Vec<Texture2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        todo!()
    }

    pub fn remove_texture(
        &mut self,
        texure_id: TextureID,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        todo!()
    }

    pub fn remove_textures(
        &mut self,
        texture_ids: Vec<TextureID>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        todo!()
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn textures(&self) -> &Vec<Texture2D> {
        &self.textures
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
