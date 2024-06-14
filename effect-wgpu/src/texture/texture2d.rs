use effect_core::id::TextureID;
use image::ImageBuffer;
use image::Rgba;

#[derive(Clone, Debug, Copy)]
pub struct TextureDescriptor2D {
    pub id: TextureID,
    pub path: &'static str,
    pub pixel_art: bool,
}

impl TextureDescriptor2D {
    pub fn new(id: TextureID, path: &'static str, pixel_art: bool) -> Self {
        Self {
            id,
            path,
            pixel_art,
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub struct Texture2D {
    id: TextureID,
    path: &'static str,
    index: u32, // u32 and not u16 as bytebuck has problems when the data types in a struct are of different size
}

impl Texture2D {
    pub fn new(id: TextureID, path: &'static str, index: u32) -> Self {
        Self { id, path, index }
    }

    pub fn file_path(&self) -> &str {
        self.path
    }

    pub fn id(&self) -> &TextureID {
        &self.id
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}
