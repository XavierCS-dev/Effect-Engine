use crate::util::file_to_bytes::file_to_bytes;

use image::{GenericImage, GenericImageView, ImageBuffer, Rgba};

use super::{
    texture2d::{Texture2D, TextureID},
    texture_pool::BindGroupID,
};

use anyhow::Result;
use image::EncodableLayout;

pub struct TextureAtlas2D {
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    textures: Vec<Texture2D>,
    atlas: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl TextureAtlas2D {
    pub fn new(mut texture: Texture2D, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
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
        let (bind_group, bind_group_layout, atlas, view, sampler) =
            Texture2D::init_texture(texture_extent, image_rgba, &device, &queue);
        texture.set_offset(0, 0);
        Self {
            textures,
            bind_group,
            bind_group_layout,
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
    ) -> Result<()> {
        let file_bytes = file_to_bytes(texture.file_path().as_str());
        let image_bytes = image::load_from_memory(file_bytes.as_bytes())
            .expect(format!("Texture {} not found", texture.file_path()).as_str());
        let image_rgba = image_bytes.to_rgba8();
        let dimensions = image_bytes.dimensions();
        let mut image_buffers: Vec<ImageBuffer<Rgba<u8>, _>> = Vec::new();
        let mut total_width = 0;
        let mut total_height = 0;

        // currently just adds textures in a line, this will need to be changed to be vertical as well.

        for tex_local in &mut self.textures {
            let file_bytes = file_to_bytes(tex_local.file_path().as_str());
            let image_bytes = image::load_from_memory(file_bytes.as_bytes())
                .expect(format!("Texture {} not found", tex_local.file_path()).as_str());
            let image_rgba = image_bytes.to_rgba8();
            let dimensions = image_bytes.dimensions();
            image_buffers.push(image_rgba);
            tex_local.set_offset(total_width, 0);
            total_width += dimensions.0;
            total_height = total_height.max(dimensions.1)
        }
        total_width += dimensions.0;
        total_height = total_height.max(dimensions.1);
        image_buffers.push(image_rgba);
        let mut combined_image = ImageBuffer::new(total_width, total_height);
        let mut current_width = 0;

        for image_rgba in image_buffers {
            let dimensions = image_rgba.dimensions();
            combined_image
                .copy_from(&image_rgba, current_width, 0)
                .unwrap();
            current_width += dimensions.0;
        }
        let extent = wgpu::Extent3d {
            width: total_width,
            height: total_height,
            depth_or_array_layers: 1,
        };

        let (bind_group, bind_group_layout, atlas, _, _) =
            Texture2D::init_texture(extent, combined_image, device, queue);
        self.atlas = atlas;
        self.bind_group = bind_group;
        self.bind_group_layout = bind_group_layout;

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

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
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
