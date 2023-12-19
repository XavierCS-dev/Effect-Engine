use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;

use super::{
    texture2d::{Texture2D, TextureID},
    texture_atlas2d::TextureAtlas2D,
};
use anyhow::Result;
use image::GenericImageView;

// Textures should be stitched together instead of storing multiple textures...
pub struct BindGroupID(u32);

pub struct TexturePool2D {
    textures: HashMap<TextureID, Texture2D>,
    bind_groups: HashMap<BindGroupID, TextureAtlas2D>,
}

impl TexturePool2D {
    pub fn new() -> Self {
        let textures = HashMap::new();
        let bind_groups = HashMap::new();
        Self {
            textures,
            bind_groups,
        }
    }

    // need to check if the bind group ID and texture ID already exist.
    // if bind group ID already exists, go to its texture atlas, and restitch the atlast together and update the texture positions.i
    // this somehow needs to be done fast as the vertices contain the texture coords, perhaps somehow another struct can be passed to wgsl
    // with the texture coords instead of the vertices, this would be much faster.
    // IF the bind group doesn't exist, create a new one, and a new texture atlast, assign all the IDs to the structs etc.
    // encourage users assign bind groups to loading area zones, and have the textures assigned to bind groups ahead of time, so they don't need to be
    // recreated. perhaps force the user to assign all textures at once, with a path list, stitch together, then create an immutable bind group.
    // path: &str becomes path: Vec<String>
    pub fn add_texture(
        &mut self,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        // if bind group ID is none, create one that is not taken.
        // if bind group exists in self.bind_groups, add to texture atlas
        // otherwise, create new texture atlas.
        todo!()
    }

    pub fn add_textures(
        &mut self,
        textures: Vec<Texture2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        todo!();
    }

    pub fn remove_texture(
        &mut self,
        texture_id: TextureID,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        todo!();
    }

    pub fn remove_textures(
        &mut self,
        texture_ids: Vec<TextureID>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        todo!();
    }
}
