use std::time::Instant;

use effect_engine::engine::{
    entity::entity::Entity2D,
    layer::layer::{Layer2DSystem, LayerID},
    primitives::vector::Vector3,
    texture::texture2d::{Texture2D, TextureID},
};
use winit::event::{Event, WindowEvent};

fn main() {
    println!("Hello, world!");
    let (mut app, event_loop) = effect_engine::init_engine();
    let mut before = Instant::now();
    let mut after = Instant::now();
    let tex_id = TextureID("tree");
    let evil_id = TextureID("evil");
    let layer_id = LayerID(1);
    let tex = app.init_texture(tex_id, "tree.png");
    let evil = app.init_texture(evil_id, "evil.png");
    let mut layer = app.init_layer(layer_id, vec![tex, evil]).unwrap();
    let position = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let ent = app.init_entity(position, tex_id, &mut layer);
    let ents = vec![ent];
    Layer2DSystem::set_entities(&mut layer, ents.as_slice(), app.device(), app.queue());
    let layers = vec![layer];
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
                app.engine.render(&layers).unwrap();
            }
            _ => (),
        }
        before = after;
    });
}
