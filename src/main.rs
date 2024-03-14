use effect_engine::{
    engine::{
        camera::camera::Camera2DSystem,
        entity::entity::Entity2D,
        layer::layer::LayerID,
        primitives::vector::Vector3,
        texture::texture2d::{Texture2D, TextureID},
    },
    EffectSystem,
};
use winit::{
    dpi::PhysicalSize,
    keyboard::{KeyCode, PhysicalKey},
};

fn main() {
    /*
    let (mut app, event_loop) = effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, true);
    let layers = Vec::new();
    EffectSystem::run(event_loop, |ctx, delta_time, control| {
        app.render(&layers).unwrap();
    })
    */
    camera_example()
}

fn camera_example() {
    let (mut app, event_loop) =
        effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, false);
    let bg_id = TextureID("grass");
    let bg = Texture2D::new(bg_id, "grass_bg_small.png");
    app.set_background(bg, true).unwrap();
    let tree_id = TextureID("tree");
    let evil_id = TextureID("evil");
    let evil_tex = Texture2D::new(evil_id, "evil.png");
    let tree_tex = Texture2D::new(tree_id, "tree.png");
    let mut tree_layer = app
        .init_layer(
            LayerID(0),
            vec![tree_tex, evil_tex],
            PhysicalSize::new(32, 32),
            true,
        )
        .unwrap();
    let tree = Entity2D::new(Vector3::new(0.0, 0.0, 0.0), &tree_layer, tree_id);
    let evil = Entity2D::new(Vector3::new(6.0, 3.0, 0.0), &tree_layer, evil_id);
    let tree_vec = vec![&tree, &evil];
    app.set_entities(&mut tree_layer, tree_vec.as_slice());
    let layers = vec![tree_layer];
    // give user access to delta time
    let mut pos = Vector3::new(0.0, 0.0, 5.0);
    EffectSystem::run(event_loop, |ctx, delta_time, control| {
        let mut cam = app.camera_mut();
        if ctx.is_key_pressed(PhysicalKey::Code(KeyCode::KeyW)) {
            pos.y += 0.005 * ((delta_time.as_micros() as f32) / 1000.0);
        }
        if ctx.is_key_pressed(PhysicalKey::Code(KeyCode::KeyS)) {
            pos.y -= 0.005 * ((delta_time.as_micros() as f32) / 1000.0);
        }
        if ctx.is_key_pressed(PhysicalKey::Code(KeyCode::KeyA)) {
            pos.x -= 0.005 * ((delta_time.as_micros() as f32) / 1000.0);
        }
        if ctx.is_key_pressed(PhysicalKey::Code(KeyCode::KeyD)) {
            pos.x += 0.005 * ((delta_time.as_micros() as f32) / 1000.0);
        }
        Camera2DSystem::transform(&mut cam, pos);
        app.update_camera();

        app.render(&layers).unwrap();
    })
}
