use std::time::Duration;

use anyhow::Result;
use effect_core::{
    camera::camera2d::Camera2D,
    id::{LayerID, TextureID},
    misc::fullscreen::FullScreenMode,
    primitives::vector::Vector3,
};
use effect_events::input::{EffectEvent, EffectEventSystem};
use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    monitor::{MonitorHandle, VideoMode},
    window::WindowBuilder,
};

use std::collections::vec_deque::*;

use crate::{
    engine::engine2d::WebEngine2D, entity::entity2d::WebEntity2D, layer::WebLayer2D,
    texture::texture2d::WebTexture2D,
};

pub struct EffectWeb2D {
    engine: WebEngine2D,
}

impl EffectWeb2D {
    pub fn new(
        screen_dimensions: PhysicalSize<u32>,
        v_sync: bool,
        app_name: &'static str,
        resizable: bool,
        fullscreen_mode: FullScreenMode,
        monitor: u32,
    ) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title(app_name)
            .with_inner_size(screen_dimensions)
            .with_resizable(resizable)
            .build(&event_loop)
            .unwrap();
        let mut monitors: Vec<MonitorHandle> = window.available_monitors().collect();
        if monitor as usize >= monitors.len() {
            panic!("Could not find monitor {}", monitor);
        }
        let monitor = monitors.remove(monitor as usize);
        let mut video_modes: Vec<VideoMode> = monitor.video_modes().collect();
        window.set_fullscreen(match fullscreen_mode {
            FullScreenMode::WINDOWED => None,
            FullScreenMode::BORDERLESS => {
                Some(winit::window::Fullscreen::Borderless(Some(monitor)))
            }
            FullScreenMode::FULLSCREEN => Some(winit::window::Fullscreen::Exclusive(
                video_modes
                    .pop()
                    .expect("Monitor does not support any video modes"),
            )),
        });
        let engine = pollster::block_on(WebEngine2D::new(window, v_sync));
        (Self { engine }, event_loop)
    }

    /// it is up to the user to sort the layers, they have the tools to do so.
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        self.engine.render()
    }

    /// Make sure your texture_size is set to the larger dimension that appears in your textures.
    /// It would be easier to use textures which all have the same dimensions
    /// and set that to the texture size, otherwise 2D transformations may not
    /// behave as you would expect them to.
    /// The maximum texture size for a layer is 8192px * 8192px
    /// The optimal stratergy is to keep similar textures on the same layer
    /// (provided you want the rendered in that order)
    /// It is advisable to have the texture_size be a square to avoid some textures getting crushed.
    pub fn init_layer(
        &mut self,
        id: LayerID,
        textures: Vec<WebTexture2D>,
        texture_size: PhysicalSize<u32>,
        pixel_art: bool,
    ) -> Result<()> {
        self.engine
            .init_layer(id, textures, texture_size, pixel_art)
    }

    pub fn set_entities(&mut self, layer: LayerID, entities: &[&WebEntity2D]) {
        self.engine.set_entities(layer, entities);
    }

    pub fn init_entity(
        &self,
        position: Vector3<f32>,
        layer: LayerID,
        texture_id: TextureID,
    ) -> WebEntity2D {
        self.engine.init_entity(position, layer, texture_id)
    }

    pub fn init_camera(&self, fov: f32) -> Camera2D {
        self.engine.init_camera(fov)
    }

    pub fn update_camera(
        &mut self,
        camera: &mut Camera2D,
        ctx: &EffectEvent,
        delta_time: Duration,
    ) {
        self.engine.update_camera(camera, ctx, delta_time);
    }

    pub fn set_background(&mut self, texture: WebTexture2D, pixel_art: bool) -> Result<()> {
        self.engine.set_background(texture, pixel_art)
    }

    pub fn update(&mut self, ctx: &mut EffectEvent, camera: &mut Option<&mut Camera2D>) {
        if ctx.window_resized() {
            self.engine.resize(ctx.window_size(), camera);
        }
        if ctx.scale_factor_changed() {
            self.engine.resize(ctx.window_size(), camera);
        }
        EffectEventSystem::reset_window_changes(ctx)
    }

    pub fn resize_window(&mut self, width: u32, height: u32) {
        let size = PhysicalSize::new(width, height);
        self.engine.window.set_resizable(true);
        let _ = self.engine.window.request_inner_size(size);
        self.engine.window.set_resizable(false);
    }

    pub fn queue(&self) -> &wgpu::Queue {
        self.engine.queue()
    }

    pub fn device(&self) -> &wgpu::Device {
        self.engine.device()
    }

    pub fn surface(&self) -> &wgpu::Surface {
        self.engine.surface()
    }
}
