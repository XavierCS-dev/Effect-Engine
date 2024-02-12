use std::time::Instant;

use effect_engine::engine::{
    entity::entity::Entity2D, layer::layer::LayerID, primitives::vector::Vector3,
    texture::texture2d::Texture2D,
};
use winit::event::{Event, WindowEvent};

fn main() {
    println!("Hello, world!");
    let (mut app, event_loop) = effect_engine::init_engine();
    let ent = app.init_entity(
        Vector3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        &app.init_texture(
            effect_engine::engine::texture::texture2d::TextureID(String::from("tree")),
            "tree.png",
        ),
        LayerID(0),
    );
    let mut before = Instant::now();
    let mut after = Instant::now();
    let _ = event_loop.run(|event, control| {
        let mut ents = Vec::new();
        ents.push(&ent);
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
                app.engine.render(vec![ents]).unwrap();
            }
            _ => (),
        }
        before = after;
    });
}
