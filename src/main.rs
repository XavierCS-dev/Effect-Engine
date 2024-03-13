use std::time::Instant;

use effect_engine::EffectSystem;
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, WindowEvent},
    keyboard::{Key, KeyCode},
};

fn main() {
    let (mut app, event_loop) =
        effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, false);
    let layers = Vec::new();
    EffectSystem::run(event_loop, |_ctx| {
        app.render(&layers).unwrap();
    })
}
