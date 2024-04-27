use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use winit::{dpi::PhysicalSize, keyboard::KeyCode};

use crate::primitives::vector::Vector3;

pub struct Camera2D {
    pub look_at: glam::Mat4,
    pub proj: glam::Mat4,
    pub position: Vector3<f32>,
    pub _near: f32,
    pub _far: f32,
    pub _fov_deg: f32,
    pub key_codes: HashMap<CameraAction, KeyCode>,
    pub current_actions: HashSet<CameraAction>,
    pub speed: f32,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CameraAction {
    Up,
    Right,
    Left,
    Down,
    ZoomIn,
    ZoomOut,
}

impl Camera2D {
    pub fn new(fov_deg: f32, aspect_ratio: f32, speed: f32) -> Self {
        let near = 0.01;
        let far = 100.0;
        let fov_rad = fov_deg.to_radians();
        let proj = glam::Mat4::perspective_rh(fov_rad, aspect_ratio, near, far);
        let look_at = glam::Mat4::look_at_rh(
            glam::Vec3::new(0.0f32, 0.0, 1.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::Y,
        );
        let key_codes = HashMap::new();
        let current_actions = HashSet::new();
        let position = Vector3::new(0.0, 0.0, 1.0);
        Self {
            proj,
            look_at,
            _near: near,
            _far: far,
            _fov_deg: fov_deg,
            key_codes,
            speed,
            current_actions,
            position,
        }
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        (self.proj * self.look_at).to_cols_array_2d()
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }

    pub fn proj(&self) -> glam::Mat4 {
        self.proj
    }

    pub fn look_at(&self) -> glam::Mat4 {
        self.look_at
    }
}

pub struct Camera2DSystem;

impl Camera2DSystem {
    pub fn update_projection(camera: &mut Camera2D, window_size: PhysicalSize<u32>) {
        let proj = glam::Mat4::perspective_rh(
            camera._fov_deg.to_radians(),
            window_size.width as f32 / window_size.height as f32,
            camera._near,
            camera._far,
        );
        camera.proj = proj;
    }

    pub fn transform(camera: &mut Camera2D, position: Vector3<f32>) {
        camera.position = position;
        camera.look_at = glam::Mat4::look_at_rh(
            glam::Vec3::new(position.x, position.y, position.z),
            glam::Vec3::new(position.x, position.y, 0.0),
            glam::Vec3::Y,
        );
    }

    pub fn set_speed(camera: &mut Camera2D, speed: f32) {
        camera.speed = speed;
    }

    pub fn set_inputs(camera: &mut Camera2D, inputs: &[(CameraAction, KeyCode)]) {
        for (action, code) in inputs {
            camera.key_codes.insert(*action, *code);
        }
    }

    pub fn remove_inputs(camera: &mut Camera2D, inputs: &[(CameraAction, KeyCode)]) {
        for (action, _) in inputs {
            let _ = camera.key_codes.remove(&action);
        }
    }

    pub fn reset_inputs(camera: &mut Camera2D) {
        camera.key_codes.clear();
    }
}
