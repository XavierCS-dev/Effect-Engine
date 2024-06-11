use effect_core::id::TextureID;
use image::ImageBuffer;
use image::Rgba;

#[derive(Clone, Debug, Copy)]
pub struct Texture2D {
    id: TextureID,
    path: &'static str,
    index: usize,
}

impl Texture2D {
    pub fn new(id: TextureID, path: &'static str, index: usize) -> Self {
        Self { id, path, index }
    }

    pub fn file_path(&self) -> &str {
        self.path
    }

    pub fn id(&self) -> &TextureID {
        &self.id
    }

    pub fn index(&self) -> usize {
        self.index
    }
}
