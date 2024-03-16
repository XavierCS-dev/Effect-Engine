use std::{fs::File, io::BufReader, time::Duration};

use effect_engine::{
    engine::{
        camera::camera::{Camera2D, Camera2DSystem, CameraAction},
        entity::entity::Entity2D,
        layer::layer::LayerID,
        primitives::vector::Vector3,
        texture::texture2d::{Texture2D, TextureID},
    },
    sound::mixer::{AudioID, Mixer, MixerSystem},
    EffectSystem,
};
use rodio::{source::SineWave, Decoder, OutputStream, Sink, Source};
use winit::{
    dpi::PhysicalSize,
    event::MouseButton,
    keyboard::{KeyCode, PhysicalKey},
};
/*
An aside my thoughts concerning f32. Currently figuring out solutions to solve accuracy problems
in a larger world. Current solution is one that is better done by users. Floating origin.
Store a list of chunks and their integer coordinates. Everything in a chunk has a local coordinate
relative the the origin of the chunk. Then each entity has a global coordinate which is relative to
the current origin. Entities chunk integer location and f32 local position can be used to calculate
positive relative to current origin. More accurate the closer the player is. The current origin
is the origin of the chunk the player is currently in.
Each time the player loads a new chunk, these coordinates will need to be recalculated.
So it is best to keep each chunk as large as possible to minimise this, and not load in all chunks of the world.
(Can use layers cache nearby unloaded chunks data to cut disk load times).

This comment will hopefully end up in the docs on the Vector3 page, I also intend to change Vector3 to generic
for floating point or perhaps integer too, will have to see.
*/

fn main() {
    sound_example();
    // camera_example();
}

// You can also use your own custom camera system by using
// Camera2DSystem::Transform. That way you can use mouse camera control,
// or move the camera only when an entity reaches the edge, etc
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
    let mut cam = app.init_camera(45.0);
    Camera2DSystem::set_inputs(
        &mut cam,
        &[
            (CameraAction::Up, KeyCode::KeyW),
            (CameraAction::Down, KeyCode::KeyS),
            (CameraAction::Left, KeyCode::KeyA),
            (CameraAction::Right, KeyCode::KeyD),
            (CameraAction::ZoomIn, KeyCode::KeyZ),
            (CameraAction::ZoomOut, KeyCode::KeyX),
        ],
    );
    Camera2DSystem::set_speed(&mut cam, 0.005);
    EffectSystem::run(event_loop, |ctx, delta_time, control| {
        if ctx.is_key_pressed(KeyCode::Escape) {
            control.exit();
        }

        Camera2DSystem::process_inputs(&mut cam, ctx, delta_time);
        app.update_camera(&mut cam);
        app.render(&layers, &cam).unwrap();
    })
}

fn sound_example() {
    /*
    "Cloud Dancer " Kevin MacLeod (incompetech.com)
    Licensed under Creative Commons: By Attribution 4.0 License
    http://creativecommons.org/licenses/by/4.0/
    */
    let (mut app, event_loop) =
        effect_engine::init_engine(PhysicalSize::new(800, 600), 45.0, false);
    let mut cam = app.init_camera(45.0);
    let layers = Vec::new();
    let mut mixer = Mixer::new();
    let track_id = AudioID("Kevin");
    MixerSystem::add_track(&mut mixer, track_id, "Cloud Dancer.mp3").unwrap();
    MixerSystem::play_track(&mixer, track_id).unwrap();
    MixerSystem::pause_track(&mixer, track_id).unwrap();
    MixerSystem::reset_track(&mut mixer, track_id).unwrap();
    MixerSystem::play_track(&mixer, track_id).unwrap();

    EffectSystem::run(event_loop, |ctx, delta_time, control| {
        if ctx.is_key_pressed(KeyCode::Escape) {
            control.exit();
        }

        app.update_camera(&mut cam);
        app.render(&layers, &cam).unwrap();
    })
}
