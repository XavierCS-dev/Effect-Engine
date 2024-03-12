pub mod engine;
use anyhow::Result;
use engine::{
    camera::{self, camera::Camera2D},
    engine as effect,
    entity::entity::Entity2D,
    layer::layer::{Layer2D, LayerID},
    primitives::vector::Vector3,
    texture::texture2d::{Texture2D, TextureID},
};
use winit::{
    dpi::PhysicalSize,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub struct EffectSystem {
    engine: effect::Engine,
}

impl EffectSystem {
    pub fn new(
        screen_dimensions: PhysicalSize<u32>,
        camera_fov: f32,
        v_sync: bool,
    ) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title("Effect Engine")
            .with_inner_size(screen_dimensions)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();
        let engine = pollster::block_on(effect::Engine::new(window, camera_fov, v_sync));
        (Self { engine }, event_loop)
    }

    /// it is up to the user to sort the layers, they have the tools to do so.
    pub fn render(&mut self, layers: &Vec<Layer2D>) -> Result<(), wgpu::SurfaceError> {
        self.engine.render(&layers)
    }

    pub fn init_texture(&self, id: TextureID, path: &'static str) -> Texture2D {
        Texture2D::new(id, path)
    }

    /// Make sure your texture_size is set to the larger dimension that appears in your textures.
    /// It would be easier to use textures which all have the same dimensions
    /// and set that to the texture size, otherwise 2D transformations may not
    /// behave as you would expect them to.
    /// The maximum texture size for a layer is 8192px * 8192px
    /// The optimal stratergy is to keep similar textures on the same layer
    /// (provided you want the rendered in that order)
    pub fn init_layer(
        &self,
        id: LayerID,
        textures: Vec<Texture2D>,
        texture_size: PhysicalSize<u32>,
        pixel_art: bool,
    ) -> Result<Layer2D> {
        self.engine
            .init_layer(id, textures, texture_size, pixel_art)
    }

    pub fn set_entities(&self, layer: &mut Layer2D, entities: &[&Entity2D]) {
        self.engine.set_entities(layer, entities);
    }

    pub fn camera(&self) -> &Camera2D {
        self.engine.camera()
    }

    pub fn camera_mut(&mut self) -> &mut Camera2D {
        self.engine.camera_mut()
    }

    pub fn update_camera(&mut self) {
        self.engine.update_camera();
    }
}

pub fn init_engine(
    screen_dimensions: PhysicalSize<u32>,
    camera_fov: f32,
    v_sync: bool,
) -> (EffectSystem, EventLoop<()>) {
    EffectSystem::new(screen_dimensions, camera_fov, v_sync)
}
