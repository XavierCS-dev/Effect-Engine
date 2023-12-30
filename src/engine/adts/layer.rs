use crate::engine::texture::{
    texture2d::Texture2D,
    texture_atlas2d::TextureAtlas2D,
    texture_pool::{BindGroupID, TexturePool2D},
};

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Debug)]
pub struct LayerID(pub u32);

pub struct Layer2D {
    id: LayerID,
    textures: Vec<Texture2D>,
    bind_group: BindGroupID,
    atlas: TextureAtlas2D,
}
