use std::time::Instant;

use effect_engine::{
    engine::{
        camera::camera::Camera2DSystem,
        entity::entity::{Entity2D, EntitySystem2D},
        layer::layer::{Layer2DSystem, LayerID},
        primitives::vector::Vector3,
        texture::texture2d::TextureID,
    },
    EffectSystem,
};
use winit::{
    dpi::PhysicalSize,
    event::{Event, WindowEvent},
};

fn main() {
    let (mut app, event_loop) =
        effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, false);
    let layers = Vec::new();
    let _ = event_loop.run(|event, control| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            control.exit();
        }
        Event::AboutToWait => {
            app.render(&layers).unwrap();
        }
        _ => (),
    });
}
