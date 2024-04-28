use effect_core::camera::camera2d::Camera2D;
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

pub struct WebCamera {
    pub bind_group: wgpu::BindGroup,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub buffer: wgpu::Buffer,
    pub proj: glam::Mat4,
    pub look_at: glam::Mat4,
}

// TODO: Going to have to figure out another way sort camera resizing
impl WebCamera {
    pub fn new(device: &wgpu::Device, proj: glam::Mat4, look_at: glam::Mat4) -> Self {
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
            bind_group,
            bind_group_layout,
            buffer,
            proj,
            look_at,
        }
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

pub struct WebCameraSystem2D;

impl WebCameraSystem2D {
    pub fn update_projection(camera: &mut WebCamera, window_size: PhysicalSize<u32>) {
        let proj = glam::Mat4::perspective_rh(
            45f32.to_radians(),
            window_size.width as f32 / window_size.height as f32,
            0.1,
            10.0,
        );
        camera.proj = proj;
    }

    pub fn update(camera: &Camera2D, web_cam: &mut WebCamera) {
        web_cam.proj = camera.proj();
        web_cam.look_at = camera.look_at();
    }

    pub fn update_buffers(camera: &WebCamera, queue: &wgpu::Queue) {
        let comp = camera.proj * camera.look_at;
        queue.write_buffer(
            &camera.buffer,
            0,
            bytemuck::cast_slice(&comp.to_cols_array()),
        );
    }
}

pub trait WebCameraBGL {
    fn layout() -> wgpu::BindGroupLayoutDescriptor<'static>;
}

impl WebCameraBGL for Camera2D {
    fn layout() -> wgpu::BindGroupLayoutDescriptor<'static> {
        wgpu::BindGroupLayoutDescriptor {
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
        }
    }
}
