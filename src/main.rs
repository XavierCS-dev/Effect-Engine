use std::time::{Duration, Instant};

use effect_engine::engine::{
    camera::camera::Camera2DSystem,
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
    let (mut app, event_loop) = effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, true);
    let mut before = Instant::now();
    let mut after = Instant::now();
    let tex_id = TextureID("tree");
    let evil_id = TextureID("evil");
    let layer_id = LayerID(1);
    let tex = app.init_texture(tex_id, "tree.png");
    let evil = app.init_texture(evil_id, "evil.png");
    let mut layer = app
        .init_layer(layer_id, vec![tex, evil], PhysicalSize::new(32, 32), true)
        .unwrap();
    let position = Vector3::new(0.0, 0.0, 0.0);
    let mut ent = Entity2D::new(position, &mut layer, evil_id);
    let mut ent_good = Entity2D::new(position, &mut layer, tex_id);
    EntitySystem2D::set_position(&mut ent_good, Vector3::new(0.25, 0.0, 0.0));
    EntitySystem2D::set_rotation(&mut ent_good, 30.0);
    EntitySystem2D::set_scale(&mut ent_good, 0.25);
    EntitySystem2D::set_scale(&mut ent, 0.2);
    let mut ents_owner = vec![ent, ent_good];
    let mut ents = Vec::new();
    for ent in ents_owner.iter() {
        ents.push(ent);
    }
    app.set_entities(&mut layer, ents.as_slice());
    drop(ents);
    let mut layers = vec![layer];
    let camera = app.camera_mut();
    Camera2DSystem::transform(camera, Vector3::new(0.35, 0.25, 1.0));
    app.update_camera();

    let mut rotation = 0.0;
    let _ = event_loop.run(|event, control| {
        after = Instant::now();
        let delta_time = after - before;
        // app.engine.input(&event, &delta_time);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control.exit();
            }
            Event::AboutToWait => {
                // app.engine.update(&delta_time);
                EntitySystem2D::set_rotation(&mut ents_owner.get_mut(1).unwrap(), rotation);
                let mut ents = Vec::new();
                for ent in ents_owner.iter() {
                    ents.push(ent);
                }
                app.set_entities(layers.get_mut(0).unwrap(), ents.as_slice());
                drop(ents);
                rotation += 0.05;
                rotation = rotation % 360.0;
                app.render(&layers).unwrap();
            }
            _ => (),
        }
        before = after;
    });
}
