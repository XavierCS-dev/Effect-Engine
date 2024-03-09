use std::time::{Duration, Instant};

use effect_engine::engine::{
    entity::entity::{Entity2D, EntitySystem2D},
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
        x: 0.2,
        y: 0.4,
        z: 0.0,
    };
    let ent = app.init_entity(position, evil_id, &mut layer);
    let mut ents = vec![ent];
    Layer2DSystem::set_entities(&mut layer, ents.as_slice(), app.device(), app.queue());
    let mut layers = vec![layer];
    let mut check = false;
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
                /*
                let new_tex;
                if check {
                    new_tex = evil_id;
                    check = false;
                    // std::thread::sleep(Duration::from_millis(200));
                } else {
                    new_tex = tex_id;
                    check = true;
                    // std::thread::sleep(Duration::from_millis(200));
                }
                EntitySystem2D::set_texture(
                    ents.first_mut().unwrap(),
                    new_tex,
                    layers.first().unwrap(),
                )
                .unwrap();
                Layer2DSystem::set_entities(
                    layers.first_mut().unwrap(),
                    ents.as_slice(),
                    app.device(),
                    app.queue(),
                );
                */
                app.engine.render(&layers).unwrap();
            }
            _ => (),
        }
        before = after;
    });
}
