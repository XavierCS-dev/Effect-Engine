#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Debug)]
#[repr(C)]
pub struct Entity2DRaw {
    pub transform: [[f32; 4]; 4],
    pub texture_index: u32,
}
