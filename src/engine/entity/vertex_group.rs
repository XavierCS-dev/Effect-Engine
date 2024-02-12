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
    ) -> Self {
        let width = (texture_width / screen_width) as f32 * 2.0 - 1.0;
        let height = (texture_height / screen_height) as f32 * 2.0 - 1.0;
        let vertices = [
            Vertex {
                position: [width, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, 0.0, 0.0],
            },
            Vertex {
                position: [0.0, height, 0.0],
            },
            Vertex {
                position: [width, height, 0.0],
            },
        ];
        Self { vertices }
    }

    pub fn vertices(&self) -> &[Vertex; 4] {
        &self.vertices
    }
}
