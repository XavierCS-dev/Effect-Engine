use super::{
    texture2d::{Texture2D, TextureID},
    texture_pool::BindGroupID,
};

use anyhow::Result;

pub struct TextureAtlas2D {
    bind_group_id: BindGroupID,
    bind_group: Option<wgpu::BindGroup>,
    bind_group_layout: Option<wgpu::BindGroupLayout>,
    texture_ids: Vec<TextureID>,
    atlas: Option<wgpu::Texture>,
    view: Option<wgpu::TextureView>,
    sampler: Option<wgpu::Sampler>,
}

impl TextureAtlas2D {
    pub fn new(bind_group_id: BindGroupID) -> Self {
        let texture_ids = Vec::new();
        Self {
            bind_group_id,
            texture_ids,
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
        /*
        move this reading to own function
        let mut file_byes: Vec<u8> = Vec::new();
        let mut file = fs::File::open(path).expect(format!("Could not find file {path}").as_str());
        file.read_to_end(&mut file_byes).unwrap();
        */
        todo!()
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

    pub fn bind_group(&self) -> &Option<wgpu::BindGroup> {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &Option<wgpu::BindGroupLayout> {
        &self.bind_group_layout
    }

    pub fn texture_ids(&self) -> &Vec<TextureID> {
        &self.texture_ids
    }

    pub fn atlas(&self) -> &Option<wgpu::Texture> {
        &self.atlas
    }

    pub fn view(&self) -> &Option<wgpu::TextureView> {
        &self.view
    }

    pub fn sampler(&self) -> &Option<wgpu::Sampler> {
        &self.sampler
    }
}
