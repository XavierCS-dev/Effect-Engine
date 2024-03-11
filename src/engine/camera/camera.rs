use wgpu::{util::DeviceExt, Queue};

use crate::engine::{
    primitives::{matrix::Matrix4, vector::Vector3},
    transform::transform::{Transform2D, Transform2DSystem},
};

pub struct Camera2D {
    proj_mat: Matrix4,
    complete_matrix: Matrix4,
    transform: Transform2D,
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
        let proj_mat = Matrix4::from_slice([
            [1.0 / (aspect_ratio * (fov_rad / 2.0).tan()), 0.0, 0.0, 0.0],
            [0.0, 1.0 / ((fov_rad / 2.0).tan()), 0.0, 0.0],
            [0.0, 0.0, far / (far - near), -(far * near) / (far - near)],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera"),
            contents: bytemuck::cast_slice(&[proj_mat]),
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
        let transform = Transform2D::new();
        let complete_matrix = Matrix4::new();
        Self {
            proj_mat,
            near,
            far,
            fov_deg,
            buffer,
            bind_group,
            bind_group_layout,
            transform,
            complete_matrix,
        }
    }

    pub fn to_raw(&self) -> [[f32; 4]; 4] {
        self.proj_mat.inner
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
        let position = Vector3::new(-position.x, -position.y, position.z);
        Transform2DSystem::translate(&mut camera.transform, position);
    }

    pub fn rotate(camera: &mut Camera2D, degrees: f32) {
        Transform2DSystem::rotate(&mut camera.transform, degrees);
    }

    pub fn update(camera: &mut Camera2D, queue: &wgpu::Queue) {
        camera.complete_matrix = camera.proj_mat * camera.transform.to_raw();
        queue.write_buffer(
            &camera.buffer,
            0,
            bytemuck::cast_slice(&camera.complete_matrix.inner),
        );
    }
}
