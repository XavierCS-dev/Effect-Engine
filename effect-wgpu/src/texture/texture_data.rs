use effect_core::id::TextureID;

pub struct TextureData {
    pub ID: TextureID,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
