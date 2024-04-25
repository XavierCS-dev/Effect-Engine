use std::{
    cmp,
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};

use effect_events::input::Context2D;
use wgpu::util::DeviceExt;
use winit::{
    event::{ElementState, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use crate::primitives::vector::Vector3;

pub struct Camera2D {
    look_at: glam::Mat4,
    proj: glam::Mat4,
    position: Vector3<f32>,
    _near: f32,
    _far: f32,
    _fov_deg: f32,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    buffer: wgpu::Buffer,
    key_codes: HashMap<CameraAction, KeyCode>,
    current_actions: HashSet<CameraAction>,
    speed: f32,
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
    pub fn new(device: &wgpu::Device, fov_deg: f32, aspect_ratio: f32, speed: f32) -> Self {
        let near = 0.01;
        let far = 100.0;
        let fov_rad = fov_deg.to_radians();
        let proj = glam::Mat4::perspective_rh(fov_rad, aspect_ratio, near, far);
        let look_at = glam::Mat4::look_at_rh(
            glam::Vec3::new(0.0f32, 0.0, 1.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            glam::Vec3::Y,
        );
        let comp = proj * look_at;

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera"),
            contents: bytemuck::cast_slice(&[comp.to_cols_array()]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });
        let key_codes = HashMap::new();
        let current_actions = HashSet::new();
        let position = Vector3::new(0.0, 0.0, 1.0);
        Self {
            proj,
            look_at,
            _near: near,
            _far: far,
            _fov_deg: fov_deg,
            buffer,
            bind_group,
            bind_group_layout,
            key_codes,
            speed,
            current_actions,
            position,
        }
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        (self.proj * self.look_at).to_cols_array_2d()
    }

    pub fn buffer(&self) -> wgpu::BufferSlice {
        self.buffer.slice(..)
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }

    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }

    pub fn position(&self) -> Vector3<f32> {
        self.position
    }
}

pub struct Camera2DSystem;

impl Camera2DSystem {
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

    pub fn update(camera: &mut Camera2D, queue: &wgpu::Queue) {
        let comp = camera.proj * camera.look_at;
        queue.write_buffer(
            &camera.buffer,
            0,
            bytemuck::cast_slice(&comp.to_cols_array()),
        );
    }

    pub fn process_inputs(camera: &mut Camera2D, ctx: &Context2D, delta_time: Duration) {
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
