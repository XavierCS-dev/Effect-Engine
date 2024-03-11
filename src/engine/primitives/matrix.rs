use std::ops::Mul;

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

impl Mul for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut new_mat = Matrix4::new();
        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    new_mat.inner[i][j] += rhs.inner[k][j] * self.inner[i][k];
                }
            }
        }
        new_mat
    }
}
