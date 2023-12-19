use image::GenericImageView;
use std::fs;
use std::io;
use std::io::prelude::*;

use super::texture_pool::BindGroupID;

pub struct TextureID(String);

// add tex coords here, make bind group mandatory.
pub struct Texture2D {
    id: TextureID,
    path: String,
    bind_group_id: Option<BindGroupID>,
    width: u32,
    height: u32,
}

impl Texture2D {
    pub fn new(id: &str, filepath: &str, device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let id = TextureID(String::from(id));
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
        }
    }
}
