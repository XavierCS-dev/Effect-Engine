use crate::engine::primitives::matrix::Matrix4;

pub struct Transform2D {
    matrix: Matrix4,
}

impl Transform2D {
    pub fn new() -> Self {
        let matrix = Matrix4::new();
        Self { matrix }
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        self.matrix.inner
    }
}

pub struct Transform2DSystem;
