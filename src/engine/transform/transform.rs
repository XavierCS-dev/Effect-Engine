use crate::engine::primitives::{matrix::Matrix4, vector::Vector3};

pub struct Transform2D {
    matrix: Matrix4,
    rotation: f32,
    scale: f32,
    position: Vector3<f32>,
}

impl Transform2D {
    pub fn new() -> Self {
        let matrix = Matrix4::new();
        let rotation = 0.0;
        let scale = 1.0;
        let position = Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        Self {
            matrix,
            rotation,
            scale,
            position,
        }
    }

    pub fn to_raw(&self) -> Matrix4 {
        self.matrix
    }

    pub fn position(&self) -> &Vector3<f32> {
        &self.position
    }

    pub fn rotation(&self) -> f32 {
        self.rotation.to_degrees()
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}

pub struct Transform2DSystem;

impl Transform2DSystem {
    pub fn rotate(transform: &mut Transform2D, degrees: f32) {
        let degrees = degrees % 360.0;
        let radians = degrees.to_radians();
        transform.rotation = radians;
        transform.matrix.inner[0][0] = radians.cos() * transform.scale;
        transform.matrix.inner[0][1] = radians.sin() * transform.scale;
        transform.matrix.inner[1][0] = -(radians.sin()) * transform.scale;
        transform.matrix.inner[1][1] = radians.cos() * transform.scale;
    }

    pub fn translate(transform: &mut Transform2D, position: Vector3<f32>) {
        transform.position = position;
        transform.matrix.inner[3][0] = position.x * transform.rotation.cos() * transform.scale
            + position.y * (-(transform.rotation.sin())) * transform.scale;
        transform.matrix.inner[3][1] = position.x * transform.rotation.sin() * transform.scale
            + position.y * transform.rotation.cos() * transform.scale;
        transform.matrix.inner[3][2] = position.z;
    }

    pub fn scale(transform: &mut Transform2D, scale: f32) {
        transform.scale = scale;
        transform.matrix.inner[0][0] = transform.rotation.cos() * scale;
        transform.matrix.inner[0][1] = transform.rotation.sin() * scale;
        transform.matrix.inner[1][0] = -(transform.rotation.sin()) * scale;
        transform.matrix.inner[1][1] = transform.rotation.cos() * scale;
    }
}
