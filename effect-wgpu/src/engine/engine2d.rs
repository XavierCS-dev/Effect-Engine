use std::{collections::BTreeMap, sync::Arc, time::Duration};

use effect_core::{
    camera::camera2d::{Camera2D, Camera2DSystem},
    id::{LayerID, TextureID},
    primitives::{vector::Vector3, vertex::Vertex},
    raw::entityraw::Entity2DRaw,
};
use effect_events::input::{camera2d::CameraUpdateSystem2D, EffectEvent};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use anyhow::Result;

use crate::{
    background::background2d::WebBackground2D,
    camera::{WebCamera, WebCameraSystem2D},
    entity::entity2d::{WebEntity2D, WebEntity2DRaw},
    layer::{WebLayer2D, WebLayer2DSystem},
    layouts::WebVertexLayout,
    texture::texture2d::WebTexture2D,
    window::{WebWindow, WebWindowSystem},
};

pub struct WebEngine2D<'a> {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub window: WebWindow<'a>,
    graphics_pipeline: wgpu::RenderPipeline,
    texture_bgl: wgpu::BindGroupLayout,
    background: Option<WebBackground2D>,
    index_buffer: wgpu::Buffer,
    camera: WebCamera,
    pub layers: BTreeMap<LayerID, WebLayer2D>,
}

/*
* Update and input should run closures defined by the user
* these closures are to be stored in Engine upon initialisation
* 0.3.0 release
*/
impl<'a> WebEngine2D<'a> {
    pub async fn new(
        device: wgpu::Device,
        queue: wgpu::Queue,
        window: WebWindow<'a>,
        graphics_pipeline: wgpu::RenderPipeline,
        texture_bgl: wgpu::BindGroupLayout,
        background: Option<WebBackground2D>,
        index_buffer: wgpu::Buffer,
        camera: WebCamera,
        layers: BTreeMap<LayerID, WebLayer2D>,
    ) -> Self {
        Self {
            device,
            queue,
            window,
            graphics_pipeline,
            texture_bgl,
            background,
            index_buffer,
            camera,
            layers,
        }
    }

    pub fn set_res(&mut self, resolution: PhysicalSize<u32>) {
        WebWindowSystem::set_resolution(&mut self.window, resolution, &self.device);
        WebCameraSystem2D::update_projection(&mut self.camera, resolution);
        WebCameraSystem2D::update_buffers(&self.camera, &self.queue);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let surface_texture = self.window.surface().get_current_texture()?;
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut command_encoder =
            self.device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Command encoder"),
                });
        let mut render_pass = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.0,
                        g: 0.5,
                        b: 0.5,
                        a: 0.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        render_pass.set_pipeline(&self.graphics_pipeline);
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        match self.background.as_ref() {
            Some(bg) => {
                render_pass.set_bind_group(0, bg.bind_group(), &[]);
                render_pass.set_bind_group(1, bg.camera_bind_group(), &[]);
                render_pass.set_vertex_buffer(0, bg.vertex_buffer());
                render_pass.set_vertex_buffer(1, bg.entity_buffer());
                render_pass.draw_indexed(0..6, 0, 0..1);
            }
            None => (),
        };

        render_pass.set_bind_group(1, self.camera.bind_group(), &[]);
        for (_, layer) in self.layers.iter() {
            render_pass.set_bind_group(0, layer.bind_group(), &[]);
            render_pass.set_vertex_buffer(0, layer.vertex_buffer());
            render_pass.set_vertex_buffer(1, layer.entity_buffer().unwrap());
            render_pass.draw_indexed(0..6 as u32, 0, 0..layer.entity_count() as u32);
        }
        drop(render_pass);
        self.queue.submit(std::iter::once(command_encoder.finish()));
        surface_texture.present();
        Ok(())
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn init_layer(
        &mut self,
        id: LayerID,
        textures: Vec<WebTexture2D>,
        texture_size: PhysicalSize<u32>,
        pixel_art: bool,
    ) -> Result<()> {
        let layer = WebLayer2D::new(
            id,
            self.window.window().inner_size(),
            textures,
            &self.device,
            &self.queue,
            &self.texture_bgl,
            texture_size,
            pixel_art,
        )?;

        let _ = self.layers.insert(id, layer);

        Ok(())
    }

    pub fn init_entity(
        &self,
        position: Vector3<f32>,
        layer: LayerID,
        texture_id: TextureID,
    ) -> WebEntity2D {
        WebEntity2D::new(position, self.layers.get(&layer).unwrap(), texture_id)
    }

    pub fn set_entities(&mut self, layer: LayerID, entities: &[&WebEntity2D]) {
        WebLayer2DSystem::set_entities(
            self.layers.get_mut(&layer).unwrap(),
            entities,
            &self.device,
            &self.queue,
        )
    }

    pub fn update_camera(
        &mut self,
        camera: &mut Camera2D,
        ctx: &EffectEvent,
        delta_time: Duration,
    ) {
        CameraUpdateSystem2D::update(camera, ctx, delta_time);
        WebCameraSystem2D::update(camera, &mut self.camera);
        WebCameraSystem2D::update_buffers(&self.camera, &self.queue)
    }

    pub fn update_camera_buffers(&mut self) {
        WebCameraSystem2D::update_buffers(&self.camera, &self.queue);
    }

    pub fn init_camera(&self, fov: f32) -> Camera2D {
        let dims = self.window.window().inner_size();
        Camera2D::new(fov, (dims.width as f32) / (dims.height as f32), 0.5)
    }

    pub fn set_background(&mut self, texture: WebTexture2D, pixel_art: bool) -> Result<()> {
        self.background = Some(WebBackground2D::new(
            texture,
            &self.texture_bgl,
            pixel_art,
            &self.device,
            &self.queue,
        )?);
        Ok(())
    }
}
