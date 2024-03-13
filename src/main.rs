use std::time::Instant;

use effect_engine::{
    engine::{
        camera::camera::Camera2DSystem,
        entity::entity::{Entity2D, EntitySystem2D},
        layer::layer::LayerID,
        primitives::vector::Vector3,
        texture::texture2d::TextureID,
    },
    EffectSystem,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
};

fn update() {
    println!("Update");
}

fn main() {
    let (mut app, event_loop) =
        effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, false);
    let layers = Vec::new();
    EffectSystem::run(event_loop, || {
        app.render(&layers).unwrap();
    });
}
