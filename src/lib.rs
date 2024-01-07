pub mod engine;
use engine::{
    adts::{entity::Entity2D, entity_group::EntityGroup2D},
    engine as effect,
};
pub mod util;
use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct EffectSystem {
    engine: effect::Engine,
}

impl EffectSystem {
    pub fn new() -> (Self, EventLoop<()>) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        let window = WindowBuilder::new()
            .with_title("Effect Engine")
            .with_inner_size(PhysicalSize::new(800, 600))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();
        let engine = pollster::block_on(effect::Engine::new(window));
        (Self { engine }, event_loop)
    }

    pub fn render(&mut self, entities: &Vec<EntityGroup2D>) {
        self.engine.render(entities).unwrap();
    }
}

pub fn init_engine() -> (EffectSystem, EventLoop<()>) {
    EffectSystem::new()
}
