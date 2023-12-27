use crate::util::file_to_bytes::file_to_bytes;

use image::GenericImageView;

use super::{
    texture2d::{Texture2D, TextureID},
    texture_pool::BindGroupID,
};

use anyhow::Result;
use image::EncodableLayout;

pub struct TextureAtlas2D {
    bind_group_id: BindGroupID,
    bind_group: Option<wgpu::BindGroup>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    textures: Vec<Texture2D>,
    atlas: Option<wgpu::Texture>,
    view: Option<wgpu::TextureView>,
    sampler: Option<wgpu::Sampler>,
}

impl TextureAtlas2D {
    pub fn new(bind_group_id: BindGroupID) -> Self {
        let textures = Vec::new();
        Self {
            bind_group_id,
            textures,
            bind_group: None,
            bind_group_layout: None,
            atlas: None,
            view: None,
            sampler: None,
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
        match self.atlas {
            None => {
                let texture_extent = wgpu::Extent3d {
                    width: dimensions.0,
                    height: dimensions.1,
                    depth_or_array_layers: 1,
                };
                let (bind_group, bind_group_layout, texture_wgpu, view, sampler) =
                    Texture2D::init_texture(
                        texture_extent,
                        image_bytes,
                        image_rgba,
                        &device,
                        &queue,
                    );
                self.bind_group = Some(bind_group);
                self.bind_group_layout = Some(bind_group_layout);
                self.view = Some(view);
                self.sampler = Some(sampler);
                self.atlas = Some(texture_wgpu);
                texture.set_offset(0, 0);
            }
            Some(texture_wgpu) => {
                todo!()
            }
        }

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
        match self.bind_group {
            Some(bind_group) => &bind_group,
            _ => panic!("Atlas not initialised! Report bug on github"),
        }
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        match self.bind_group_layout {
            Some(layout) => &layout,
            _ => panic!("Atlas not initialised! Report bug on github"),
        }
    }

    pub fn textures(&self) -> &Vec<Texture2D> {
        &self.textures
    }

    pub fn atlas(&self) -> &wgpu::Texture {
        match self.atlas {
            Some(atlas) => &atlas,
            _ => panic!("Atlas not initialised! Report bug on github"),
        }
    }

    pub fn view(&self) -> &wgpu::TextureView {
        match self.view {
            Some(view) => &view,
            _ => panic!("Atlas not initialised! Report bug on github"),
        }
    }

    pub fn sampler(&self) -> &wgpu::Sampler {
        match self.sampler {
            Some(sampler) => &sampler,
            _ => panic!("Atlas not initialised! Report bug on github"),
        }
    }
}
