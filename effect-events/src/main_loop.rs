use std::time::{Duration, Instant};

use winit::{
    event::Event,
    event_loop::{EventLoop, EventLoopWindowTarget},
};

use crate::input::{Context2D, Context2DSystem};

pub struct EffectEventLoop {}

impl EffectEventLoop {
    pub fn run<F>(event_loop: EventLoop<()>, mut user_loop: F)
    where
        F: FnMut(&Context2D, Duration, &EventLoopWindowTarget<()>) -> (),
    {
        let mut ctx = Context2D::new();
        let mut before = Instant::now();
        let mut after = Instant::now();
        let _ = event_loop.run(|event, control| {
            // As more systems are updated here and get more complex,
            // these updated can be run in a worker task multithreading fashion
            // before joining back together
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
