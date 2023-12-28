use crate::util::file_to_bytes::file_to_bytes;

use image::{GenericImageView, ImageBuffer};

use super::{
    texture2d::{Texture2D, TextureID},
    texture_pool::BindGroupID,
};

use anyhow::Result;
use image::EncodableLayout;

pub struct TextureAtlas2D {
    bind_group_id: BindGroupID,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    textures: Vec<Texture2D>,
    atlas: wgpu::Texture,
    view: wgpu::TextureView,
    sampler: wgpu::Sampler,
}

impl TextureAtlas2D {
    pub fn new(
        bind_group_id: BindGroupID,
        mut texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
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
        let (bind_group, bind_group_layout, atlas, view, sampler) =
            Texture2D::init_texture(texture_extent, image_bytes, image_rgba, &device, &queue);
        texture.set_offset(0, 0);
        Self {
            bind_group_id,
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

        /*
        Takes all the textures from the texture vec, and the new texture, read them into memory,
        then stitch then all together, making sure to set their offsets, then recreate the atlas
        bindgroup, view and sampler.
        */

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

    pub fn bind_group_id(&self) -> &BindGroupID {
        &self.bind_group_id
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
