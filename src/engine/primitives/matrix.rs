#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Matrix4 {
    pub inner: [[f32; 4]; 4],
}

impl Matrix4 {
    /// Returns the identity matrix
    pub fn new() -> Self {
        let inner = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Self { inner }
    }

    pub fn from_slice(mat_slice: [[f32; 4]; 4]) -> Self {
        Self { inner: mat_slice }
    }
}
