#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Debug, Copy)]
pub struct BackgroundID(pub &'static str);
#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Debug, Copy)]
pub struct TextureID(pub &'static str);

#[derive(std::cmp::PartialEq, std::cmp::Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
pub struct LayerID(pub u32);
