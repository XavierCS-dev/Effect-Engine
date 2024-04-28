use std::time::Duration;

use anyhow::Result;
use effect_core::{
    camera::camera2d::Camera2D,
    id::{LayerID, TextureID},
    misc::{fullscreen::FullScreenMode, window_info::WindowInfo},
    primitives::vector::Vector3,
};
use effect_events::input::{EffectEvent, EffectEventSystem};
use winit::{
    dpi::PhysicalSize,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    monitor::{MonitorHandle, VideoMode},
};

use std::collections::vec_deque::*;

use crate::{
    engine::engine2d::WebEngine2D, entity::entity2d::WebEntity2D, layer::WebLayer2D,
    texture::texture2d::WebTexture2D,
};

pub struct EffectWeb2D<'a> {
    engine: WebEngine2D<'a>,
}

impl<'a> EffectWeb2D<'a> {
    pub fn new(
        engine: WebEngine2D<'a>, /*
                                 screen_dimensions: PhysicalSize<u32>,
                                 app_name: &'static str,
                                 resizable: bool,
                                 fullscreen_mode: FullScreenMode,
                                 monitor: u32,
                                 */
    ) -> Self {
        Self { engine }
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

    pub fn update(&mut self, ctx: &mut EffectEvent) {
        /*
        if ctx.window_resized() {
            println!("resized");
            self.engine.set_res(ctx.window_size());
        }
        EffectEventSystem::reset_window_changes(ctx);
        */
    }

    pub fn set_resolution(&mut self, width: u32, height: u32) {
        let resolution = PhysicalSize::new(width, height);
        self.engine.set_res(resolution);
    }
}
