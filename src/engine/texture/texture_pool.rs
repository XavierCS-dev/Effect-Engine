use rand;
use rand::Rng;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::prelude::*;
use wgpu::BindGroup;
use wgpu::Texture;

use super::{
    texture2d::{Texture2D, TextureID},
    texture_atlas2d::TextureAtlas2D,
};
use anyhow::Result;
use image::GenericImageView;

// Textures should be stitched together instead of storing multiple textures...
#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug)]
pub struct BindGroupID(pub u32);

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
    ) -> Result<BindGroupID> {
        // if bind group ID is none, create one that is not taken.
        // if bind group exists in self.bind_groups, add to texture atlas
        // otherwise, create new texture atlas.
        if self.textures.contains_key(&texture.id()) {
            panic!("Texture already exists {:?}", texture.id());
            // return an Err instead later
        }
        let mut rng = rand::thread_rng();
        let mut id: BindGroupID;
        let mut texture = texture;
        match texture.bind_group_id() {
            Some(id_in) => {
                id = id_in;
            }
            None => {
                /*
                may not always be random, there will be the concept of layers
                this allows for drawing object on top of each other in the correct order,
                can allow for y sorting into layers too
                one draw call per layer, 8096x8096 pixel texture limit per layer
                not sure how this will relate to bind group ID however, perhaps
                the plan should be to set the layer in the texture atlas, and use that for draw order.
                Perhaps layer should superscede BingGroupID
                */
                id = BindGroupID(rng.gen());
                while self.bind_groups.contains_key(&id) {
                    id = BindGroupID(rng.gen());
                }
                texture.set_bind_group_id(id);
            }
        };
        let local_atlas = self.bind_groups.get_mut(&id);
        if local_atlas.is_none() {
            // create new texture atlas
            let local_atlas = TextureAtlas2D::new(
                texture.bind_group_id().unwrap(),
                texture.clone(),
                device,
                queue,
            );
            self.bind_groups
                .insert(texture.bind_group_id().unwrap(), local_atlas);
            self.textures.insert(texture.id().clone(), texture);
        } else {
            let local_atlas = local_atlas.unwrap();
            let _ = local_atlas
                .add_texture(texture.clone(), device, queue)
                .unwrap();
            self.textures.insert(texture.id().clone(), texture);
        }
        Ok(id)
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

    pub fn get_atlas(&self, id: BindGroupID) -> Option<&TextureAtlas2D> {
        self.bind_groups.get(&id)
    }
}
