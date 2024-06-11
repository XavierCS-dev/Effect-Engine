use effect_core::id::TextureID;

pub struct TextureData {
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}
