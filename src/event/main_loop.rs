use std::time::{Duration, Instant};

use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    keyboard::KeyCode,
};

use crate::{engine::camera::camera::Camera2DSystem, EffectSystem};

use super::input::context::{Context2D, Context2DSystem};

impl EffectSystem {
    pub fn run<F>(event_loop: EventLoop<()>, mut user_loop: F)
    where
        F: FnMut(&Context2D, Duration, &EventLoopWindowTarget<()>) -> (),
    {
        let mut ctx = Context2D::new();
        let mut before = Instant::now();
        let mut after = Instant::now();
        let _ = event_loop.run(|event, control| {
            Context2DSystem::update(&mut ctx, &event);
            match event {
                Event::AboutToWait => {
                    after = Instant::now();
                    user_loop(&ctx, after - before, control);
                    before = after;
                    Context2DSystem::clear_released(&mut ctx);
                }
                _ => (),
            }
        });
    }
}
