use std::time::{Duration, Instant};

use effect_engine::engine::{
    entity::entity::{Entity2D, EntitySystem2D},
    layer::layer::{Layer2DSystem, LayerID},
    primitives::vector::Vector3,
    texture::texture2d::{Texture2D, TextureID},
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
};

fn main() {
    println!("Hello, world!");
    let (mut app, event_loop) = effect_engine::init_engine(PhysicalSize::new(800, 600));
    let mut before = Instant::now();
    let mut after = Instant::now();
    let tex_id = TextureID("tree");
    let evil_id = TextureID("evil");
    let bob_id = TextureID("bob");
    let layer_id = LayerID(1);
    let tex = app.init_texture(tex_id, "tree.png");
    let evil = app.init_texture(evil_id, "evil.png");
    let bob = app.init_texture(bob_id, "bob.png");
    let mut layer = app
        .init_layer(layer_id, vec![tex, evil, bob], PhysicalSize::new(64, 64))
        .unwrap();
    let position = Vector3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let position_g = Vector3 {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    let position_b = Vector3 {
        x: -1.0,
        y: -1.0,
        z: 0.0,
    };
    let ent = app.init_entity(position, tex_id, &mut layer);
    let ent_good = app.init_entity(position_g, evil_id, &mut layer);
    let bob_ent = app.init_entity(position_b, bob_id, &mut layer);
    let mut ents_owner = vec![ent, ent_good, bob_ent];
    for _ in 0..10000 {
        ents_owner.push(app.init_entity(position_b, evil_id, &mut layer));
    }
    let mut ents = Vec::new();
    for ent in ents_owner.iter() {
        ents.push(ent);
    }
    Layer2DSystem::set_entities(&mut layer, ents.as_slice(), app.device(), app.queue());
    let layers = vec![layer];
    let _check = false;
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
