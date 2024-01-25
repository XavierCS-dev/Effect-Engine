use crate::engine::texture::texture2d::Texture2D;

pub struct VertexGroup2D {
    vertices: [f32; 4],
    indices: [f32; 6],
}

impl VertexGroup2D {
    pub fn new(texture: &Texture2D) -> Self {
        todo!()
    }

    pub fn vertices(&self) -> &[f32; 4] {
        &self.vertices
    }

    pub fn indices(&self) -> &[f32; 6] {
        &self.indices
    }
}
