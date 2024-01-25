use std::time::Instant;

use winit::event::{Event, WindowEvent};

fn main() {
    println!("Hello, world!");
    let (mut app, event_loop) = effect_engine::init_engine();
    let mut before = Instant::now();
    let mut after = Instant::now();
    let _ = event_loop.run(|event, control| {
        after = Instant::now();
        let delta_time = after - before;
        app.engine.input(&event, &delta_time);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control.exit();
            }
            Event::AboutToWait => {
                app.engine.update(&delta_time);
                // Should probably handle this somewhere..
                // app.engine.render().unwrap();
            }
            _ => (),
        }
        before = after;
    });
}
