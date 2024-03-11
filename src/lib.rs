pub mod engine;
use anyhow::Result;
use engine::{
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
    pub engine: effect::Engine,
}

impl EffectSystem {
    pub fn new(screen_dimensions: PhysicalSize<u32>) -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title("Effect Engine")
            .with_inner_size(screen_dimensions)
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();
        let engine = pollster::block_on(effect::Engine::new(window));
        (Self { engine }, event_loop)
    }

    /// it is up to the user to sort the layers, they have the tools to do so.
    pub fn render(&mut self, layers: &Vec<Layer2D>) -> Result<(), wgpu::SurfaceError> {
        self.engine.render(&layers)
    }

    pub fn init_entity(
        &mut self,
        position: Vector3,
        texture_id: TextureID,
        layer: &mut Layer2D,
    ) -> Entity2D {
        self.engine.init_entity(position, texture_id, layer)
    }

    pub fn init_texture(&self, id: TextureID, path: &'static str) -> Texture2D {
        Texture2D::new(id, path, self.engine.device(), self.engine.queue())
    }

    pub fn init_layer(
        &self,
        id: LayerID,
        textures: Vec<Texture2D>,
        texture_size: PhysicalSize<u32>,
    ) -> Result<Layer2D> {
        self.engine.init_layer(id, textures, texture_size)
    }

    pub fn device(&self) -> &wgpu::Device {
        self.engine.device()
    }

    pub fn queue(&self) -> &wgpu::Queue {
        self.engine.queue()
    }
}

pub fn init_engine(screen_dimensions: PhysicalSize<u32>) -> (EffectSystem, EventLoop<()>) {
    EffectSystem::new(screen_dimensions)
}
