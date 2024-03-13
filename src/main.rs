use std::time::Instant;

use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, WindowEvent},
    keyboard::{Key, KeyCode},
};

fn main() {
    let (mut app, event_loop) =
        effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, false);
    let layers = Vec::new();
    let _ = event_loop.run(|event, control| match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput { event, .. } => {
                if event.state == ElementState::Pressed {
                    if event.physical_key == KeyCode::Escape {
                        control.exit();
                    }
                }
            }
            _ => (),
        },
        Event::AboutToWait => {
            app.render(&layers).unwrap();
        }
        _ => (),
    });
}
