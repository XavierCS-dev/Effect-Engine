use winit::dpi::PhysicalSize;

use crate::engine::primitives::vertex::Vertex;
use crate::engine::texture::texture2d::Texture2D;

/// Calculates the vertices and indices for each image texture
pub struct VertexGroup2D {
    vertices: [Vertex; 4],
}

impl VertexGroup2D {
    pub fn new(
        texture_width: u32,
        texture_height: u32,
        screen_width: u32,
        screen_height: u32,
        atlas_dimensions: PhysicalSize<u32>,
        texture_offset: [u32; 2],
    ) -> Self {
        // remove * 10.0 when done
        let width = (texture_width as f32 / screen_width as f32) * 10.0;
        let height = (texture_height as f32 / screen_height as f32) * 10.0;
        let tex_w1 = texture_offset[0] as f32 / atlas_dimensions.width as f32;
        let tex_h1 = texture_offset[1] as f32 / atlas_dimensions.height as f32;
        let tex_w2 = tex_w1 + (texture_width as f32 / atlas_dimensions.width as f32);
        let tex_h2 = tex_h1 + (texture_height as f32 / atlas_dimensions.height as f32);
        let vertices = [
            Vertex {
                position: [width, height, 0.0],
                tex_coords: [tex_w2, tex_h1],
            },
            Vertex {
                position: [0.0, height, 0.0],
                tex_coords: [tex_w1, tex_h1],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
                tex_coords: [tex_w1, tex_h2],
            },
            Vertex {
                position: [width, 0.0, 0.0],
                tex_coords: [tex_w2, tex_h2],
            },
        ];
        Self { vertices }
    }

    pub fn vertices(&self) -> &[Vertex; 4] {
        &self.vertices
    }
}
