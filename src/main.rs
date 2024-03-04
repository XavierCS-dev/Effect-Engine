use std::time::Instant;

use effect_engine::engine::{
    entity::entity::Entity2D, layer::layer::LayerID, primitives::vector::Vector3,
    texture::texture2d::Texture2D,
};
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
                app.engine.render(Vec::new()).unwrap();
            }
            _ => (),
        }
        before = after;
    });
}
