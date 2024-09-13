use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use effect_core::{
    camera::camera2d::{Camera2D, CameraAction},
    primitives::vector::Vector3,
};
use winit::keyboard::KeyCode;

use super::EffectEvent;

pub struct CameraUpdateSystem2D;

impl CameraUpdateSystem2D {
    pub fn update(camera: &mut Camera2D, ctx: &EffectEvent, delta_time: Duration) {
        for (camera_action, key_code) in camera.key_codes.iter() {
            if ctx.is_key_pressed(*key_code) {
                camera.current_actions.insert(*camera_action);
            }
            if ctx.is_key_released(*key_code) {
                camera.current_actions.remove(camera_action);
            }
        }
        let dt = delta_time.as_micros() as f32 / 1000.0;
        for action in camera.current_actions.iter() {
            match action {
                CameraAction::Up => {
                    camera.position.y += camera.speed * dt;
                }
                CameraAction::Down => {
                    camera.position.y -= camera.speed * dt;
                }
                CameraAction::Right => {
                    camera.position.x += camera.speed * dt;
                }
                CameraAction::Left => {
                    camera.position.x -= camera.speed * dt;
                }
                CameraAction::ZoomIn => {
                    camera.position.z -= camera.speed * dt;
                }
                CameraAction::ZoomOut => {
                    camera.position.z += camera.speed * dt;
                }
            }
        }

        camera.look_at = glam::Mat4::look_at_rh(
            glam::Vec3::new(camera.position.x, camera.position.y, camera.position.z),
            glam::Vec3::new(camera.position.x, camera.position.y, 0.0),
            glam::Vec3::Y,
        );
    }
}
