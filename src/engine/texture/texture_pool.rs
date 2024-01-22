use std::collections::btree_map::IntoIter;
use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::engine::adts::layer::Layer2D;
use crate::engine::adts::layer::LayerID;
use crate::engine::traits::layer::Layer;

use super::texture2d::{Texture2D, TextureID};
use anyhow::Result;
use rayon::array::IntoIter;

// Textures should be stitched together instead of storing multiple textures...
#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug)]
pub struct BindGroupID(pub u32);

pub struct TexturePool2D {
    layers: BTreeMap<LayerID, Layer2D>,
}

impl TexturePool2D {
    pub fn new() -> Self {
        let layers = BTreeMap::new();
        Self { layers }
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
        layer_id: LayerID,
        texture: Texture2D,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<()> {
        if self.layers.contains_key(&layer_id) {
            panic!("Layer ID Already in use!");
        }

        match &mut self.layers.get_mut(&layer_id) {
            Some(layer) => {
                layer.add_texture(texture, device, queue)?;
            }
            None => {
                let layer = Layer2D::new(layer_id, texture, device, queue)?;
                self.layers.insert(layer_id, layer);
            }
        };

        Ok(())
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
        layer_id: LayerID,
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

    pub fn get_layer(&self, layer_id: LayerID) -> Option<&Layer2D> {
        self.layers.get(&layer_id)
    }

    pub fn get_layers(&self) -> &BTreeMap<LayerID, Layer2D> {
        &self.layers
    }
}
