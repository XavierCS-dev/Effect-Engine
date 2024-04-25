#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
pub struct Entity2DRaw {
    pub transform: [[f32; 4]; 4],
    pub texture_index: [f32; 2],
    pub texture_size: [f32; 2],
}
