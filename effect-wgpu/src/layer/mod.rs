use anyhow::Result;
use effect_core::{
    id::{LayerID, TextureID},
    primitives::vertex::Vertex,
    raw::entityraw::Entity2DRaw,
};
use std::collections::{hash_map::Keys, HashMap};
use winit::dpi::PhysicalSize;

use crate::{
    entity::entity2d::WebEntity2D,
    texture::{texture2d::WebTexture2D, texture_atlas::WebTextureAtlas2D},
};

// Takes final ownership of textures, the data etc.
// When a entity wants to get the texture offset, it must get the data from here.
pub struct WebLayer2D {
    id: LayerID,
    textures: HashMap<TextureID, WebTexture2D>,
    atlas: WebTextureAtlas2D,
    vertex_buffer: wgpu::Buffer,
    entity_count: usize,
    entity_maximum: usize,
    entity_buffer: Option<wgpu::Buffer>,
    dimensions: winit::dpi::PhysicalSize<u32>,
}

impl WebLayer2D {
    pub fn new(
        id: LayerID,
        dimensions: winit::dpi::PhysicalSize<u32>,
        mut textures: Vec<WebTexture2D>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        texture_size: PhysicalSize<u32>,
        pixel_art: bool,
    ) -> Result<Self> {
        let atlas = WebTextureAtlas2D::new(
            &mut textures,
            device,
            queue,
            bind_group_layout,
            texture_size,
            pixel_art,
        )?;
        let mut textures_layer = HashMap::new();
        for texture in textures {
            textures_layer.insert(texture.id().clone(), texture);
        }
        let entity_count = 0;
        let entity_maximum = 0;
        let vertex_buffer = WebLayer2DSystem::create_vertex_buffer(
            texture_size,
            atlas.tex_coord_size(),
            device,
            queue,
        );
        Ok(Self {
            id,
            textures: textures_layer,
            atlas,
            vertex_buffer,
            entity_count,
            entity_buffer: None,
            entity_maximum,
            dimensions,
        })
    }

    pub fn tex_coord_size(&self) -> PhysicalSize<f32> {
        self.atlas.tex_coord_size()
    }

    pub fn id(&self) -> LayerID {
        self.id
    }

    pub fn contains_texture(&self, texture_id: &TextureID) -> bool {
        self.textures.contains_key(texture_id)
    }

    pub fn texture_ids<'a>(&'a self) -> Keys<'a, TextureID, WebTexture2D> {
        self.textures.keys()
    }

    pub fn vertex_buffer(&self) -> wgpu::BufferSlice {
        self.vertex_buffer.slice(..)
    }

    pub fn entity_buffer(&self) -> Option<wgpu::BufferSlice> {
        match self.entity_buffer.as_ref() {
            Some(e_buf) => {
                let length = self.entity_count * std::mem::size_of::<Entity2DRaw>();
                Some(e_buf.slice(0..length as u64))
            }
            None => None,
        }
    }

    pub fn index_count(&self) -> usize {
        self.entity_count * 6
    }

    pub fn entity_count(&self) -> usize {
        self.entity_count
    }

    pub fn entity_maximum(&self) -> usize {
        self.entity_maximum
    }

    pub fn get_texture(&self, id: TextureID) -> Option<&WebTexture2D> {
        self.textures.get(&id)
    }

    pub fn width(&self) -> u32 {
        self.dimensions.width
    }

    pub fn height(&self) -> u32 {
        self.dimensions.height
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        self.atlas.bind_group()
    }

    pub fn atlas_dimensions(&self) -> PhysicalSize<u32> {
        self.atlas.dimensions()
    }
}

pub struct WebLayer2DSystem;

impl WebLayer2DSystem {
    fn alloc_buffer(
        data: &[u8],
        size: u64,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        label: &str,
        index: bool,
    ) -> wgpu::Buffer {
        let usage;
        if index {
            usage = wgpu::BufferUsages::INDEX;
        } else {
            usage = wgpu::BufferUsages::VERTEX;
        }
        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&buffer, 0, data);
        buffer
    }

    // alloc buffer to 2x the size and set new max entity count
    fn create_entity_buffer(
        entities: &[&WebEntity2D],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::Buffer {
        let ents = entities.iter().map(|e| e.to_raw()).collect::<Vec<_>>();
        let data: &[u8] = bytemuck::cast_slice(ents.as_slice());
        let size = std::mem::size_of_val(data) as u64;
        WebLayer2DSystem::alloc_buffer(data, size * 2, device, queue, "Entity Buffer", false)
    }

    fn _create_index_buffer(device: &wgpu::Device, queue: &wgpu::Queue) -> wgpu::Buffer {
        let mut indices: Vec<u16> = Vec::new();
        indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
        let data: &[u8] = bytemuck::cast_slice(&indices);
        let size = (std::mem::size_of::<u16>() * 6) as u64;
        WebLayer2DSystem::alloc_buffer(data, size, device, queue, "Index Buffer", true)
    }

    fn create_vertex_buffer(
        _texture_size: PhysicalSize<u32>,
        tex_coord_size: PhysicalSize<f32>,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> wgpu::Buffer {
        // Vrushing bug comes from here, figure out how to calculate these properly.
        let width = 0.5;
        let height = 0.5;
        let verts = vec![
            Vertex {
                position: [width, height, 0.0],
                tex_coords: [tex_coord_size.width, 0.0],
            },
            Vertex {
                position: [-width, height, 0.0],
                tex_coords: [0.0, 0.0],
            },
            Vertex {
                position: [-width, -height, 0.0],
                tex_coords: [0.0, tex_coord_size.height],
            },
            Vertex {
                position: [width, -height, 0.0],
                tex_coords: [tex_coord_size.width, tex_coord_size.height],
            },
        ];
        let data: &[u8] = bytemuck::cast_slice(verts.as_slice());
        let size = (std::mem::size_of::<Vertex>() * verts.len()) as u64;
        WebLayer2DSystem::alloc_buffer(data, size, device, queue, "Vertex Buffer", false)
    }

    /// Set the entity data for the particular layer.
    /// Ensure every entity has a texture from the specified layer otherwise you will run into problems.
    pub fn set_entities(
        layer: &mut WebLayer2D,
        entities: &[&WebEntity2D],
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        // allocating exactly amount needed each time may increase the number of allocations needed..
        // perhaps a strategy of allocatin 2X needed data would be better
        layer.entity_count = entities.len();

        if layer.entity_count() > layer.entity_maximum() || layer.entity_buffer().is_none() {
            // Allocate new buffers
            layer.entity_buffer = Some(WebLayer2DSystem::create_entity_buffer(
                &entities, device, queue,
            ));
            layer.entity_maximum = layer.entity_count * 2;
        } else {
            // Reuse buffers
            let ents = entities.iter().map(|e| e.to_raw()).collect::<Vec<_>>();
            let entity_data: &[u8] = bytemuck::cast_slice(ents.as_slice());
            queue.write_buffer(&layer.entity_buffer.as_ref().unwrap(), 0, entity_data);
        }
    }
}
