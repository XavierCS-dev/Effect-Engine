use effect_engine::{
    engine::{
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
    let (mut app, event_loop) =
        effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, false);
    let bg_id = TextureID("grass");
    let bg = Texture2D::new(bg_id, "grass_bg_small.png");
    app.set_background(bg, true).unwrap();
    let tree_id = TextureID("tree");
    let tree_tex = Texture2D::new(tree_id, "tree.png");
    let mut tree_layer = app
        .init_layer(LayerID(0), vec![tree_tex], PhysicalSize::new(32, 32), true)
        .unwrap();
    let tree = Entity2D::new(Vector3::new(0.0, 0.0, -1.0), &tree_layer, tree_id);
    let tree_vec = vec![&tree];
    app.set_entities(&mut tree_layer, tree_vec.as_slice());
    let layers = vec![tree_layer];
    // give user access to delta time
    EffectSystem::run(event_loop, |ctx, delta_time, control| {
        app.render(&layers).unwrap();
        if ctx.is_key_pressed(PhysicalKey::Code(KeyCode::KeyA))
            && ctx.is_key_pressed(PhysicalKey::Code(KeyCode::KeyD))
        {
            println!("Hello, world");
        }
        if ctx.is_key_released(PhysicalKey::Code(KeyCode::KeyA)) {
            println!("Goodbye world");
        }
        if ctx.is_key_pressed(PhysicalKey::Code(KeyCode::Escape)) {
            control.exit();
        }
    })
}
