pub mod engine;
use engine::{
    adts::{entity::Entity2D, entity_group::EntityGroup2D, layer::LayerID},
    engine as effect,
};
pub mod util;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
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

    pub fn render(&mut self, mut entities: Vec<&Entity2D>) {
        entities.sort_unstable_by_key(|v| v.layer_id().0);
        let mut layers: Vec<Vec<&Entity2D>> = Vec::new();
        layers.push(Vec::new());
        layers.get_mut(0).unwrap().push(entities.first().unwrap());
        let mut last = entities.first().unwrap().layer_id().0;
        entities.remove(0);
        let mut index = 0;
        for entity in entities {
            if entity.layer_id().0 != last {
                layers.push(Vec::new());
                index += 1;
                last = entity.layer_id().0;
            }
            layers.get_mut(index).unwrap().push(entity);
        }

        self.engine.render(layers).unwrap();
    }
}

pub fn init_engine() -> (EffectSystem, EventLoop<()>) {
    EffectSystem::new()
}
