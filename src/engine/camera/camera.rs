use std::f32::consts::PI;

use wgpu::{util::DeviceExt, Queue};

use crate::engine::{
    primitives::{matrix::Matrix4, vector::Vector3},
    transform::transform::{Transform2D, Transform2DSystem},
};

pub struct Camera2D {
    look_at: glam::Mat4,
    proj: glam::Mat4,
    near: f32,
    far: f32,
    fov_deg: f32,
    bind_group: wgpu::BindGroup,
    bind_group_layout: wgpu::BindGroupLayout,
    buffer: wgpu::Buffer,
}

impl Camera2D {
    pub fn new(device: &wgpu::Device, fov_deg: f32, aspect_ratio: f32) -> Self {
        let near = 0.01;
        let far = 10.0;
        let fov_rad = fov_deg.to_radians();
        let h = 1.0 / (fov_rad / 2.0).tan();
        let inv = 1.0 / aspect_ratio;
        let a = far / (far - near);
        let b = -(near * far) / (far - near);
        /*
        let proj_mat = Matrix4::from_slice([
            [h * inv, 0.0, 0.0, 0.0],
            [0.0, h, 0.0, 0.0],
            [0.0, 0.0, a, 1.0],
            [0.0, 0.0, b, 0.0],
        ]);
        */
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
        Self {
            proj,
            look_at,
            near,
            far,
            fov_deg,
            buffer,
            bind_group,
            bind_group_layout,
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
}

pub struct Camera2DSystem;

impl Camera2DSystem {
    pub fn transform(camera: &mut Camera2D, position: Vector3) {
        camera.look_at = glam::Mat4::look_at_rh(
            glam::Vec3::new(position.x, position.y, position.z),
            glam::Vec3::new(position.x, position.y, 0.0),
            glam::Vec3::Y,
        );
    }

    pub fn rotate(camera: &mut Camera2D, degrees: f32) {
        todo!()
    }

    pub fn update(camera: &mut Camera2D, queue: &wgpu::Queue) {
        let comp = camera.proj * camera.look_at;
        queue.write_buffer(
            &camera.buffer,
            0,
            bytemuck::cast_slice(&comp.to_cols_array()),
        );
    }
}
