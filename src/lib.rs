pub mod engine;
use engine::engine as effect;
use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

struct WGPUApp {
    engine: effect::Engine,
}

impl WGPUApp {
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

    pub fn run(&mut self, event_loop: EventLoop<()>) {
        // calculate delta time and pass it to update and input
        // delta time is broken..fixed it somehow
        let mut before = Instant::now();
        let mut after = Instant::now();
        let _ = event_loop.run(|event, control| {
            after = Instant::now();
            let delta_time = after - before;
            self.engine.input(&event, &delta_time);
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    control.exit();
                }
                Event::AboutToWait => {
                    self.engine.update(&delta_time);
                    // Should probably handle this somewhere..
                    self.engine.render().unwrap();
                }
                _ => (),
            }
            before = after;
        });
    }
}

pub fn init_engine() {
    let (mut app, event_loop) = WGPUApp::new();
    app.run(event_loop);
}
