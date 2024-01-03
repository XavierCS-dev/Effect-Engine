use crate::engine::adts::layer::LayerID;

pub trait Layer {
    fn bind_group(&self) -> &wgpu::BindGroup;

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;

    fn id(&self) -> LayerID;
}
