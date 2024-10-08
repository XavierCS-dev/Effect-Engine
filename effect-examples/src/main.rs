use effect_engine::core::camera::camera2d::{Camera2D, Camera2DSystem, CameraAction};
use effect_engine::core::id::{LayerID, TextureID};
use effect_engine::core::misc::fullscreen::FullScreenMode;
use effect_engine::core::primitives::vector::Vector3;
use effect_engine::events::input::camera2d::CameraUpdateSystem2D;
use effect_engine::web_render::app::effect2d::EffectEngine2D;
use effect_engine::web_render::texture::texture2d::Texture2D;
use effect_engine::EffectAppBuilder;
use winit::dpi::PhysicalSize;
use winit::keyboard::KeyCode;

struct GameState {
    initialised: bool,
    camera: Option<Camera2D>,
}

impl GameState {
    pub fn initialise(&mut self, app: &mut EffectEngine2D) {
        let tex_id = TextureID("Tree");
        let texture = Texture2D::new(tex_id, "assets/tree.png");
        let tex = vec![texture];
        app.init_layer(LayerID(0), tex, PhysicalSize::new(32, 32), true)
            .unwrap();
        let ent = app.init_entity(Vector3::new(0.0, 0.0, -1.0), LayerID(0), tex_id);
        let ents = vec![&ent];
        app.set_entities(LayerID(0), &ents);
        let mut camera = app.init_camera(90.0);
        Camera2DSystem::set_inputs(
            &mut camera,
            &[
                (CameraAction::Left, KeyCode::KeyA),
                (CameraAction::Right, KeyCode::KeyD),
            ],
        );
        Camera2DSystem::set_speed(&mut camera, 0.02);
        self.camera = Some(camera);
        self.initialised = true;
    }

    pub fn initialised(&self) -> bool {
        self.initialised
    }
}

// TODO: Simplifiy initialisation of parts.
// TODO: Reduce dependency on app for initialisation,
// so user can do their init first
fn main() {
    let event_loop = EffectAppBuilder::default()
        .fullscreen_mode(FullScreenMode::BORDERLESS)
        .resolution(1280, 720)
        .vsync(false)
        .monitor(0)
        .build()
        .get_wgpu_2d();

    // Extremely verbose just to get a texture on screen.
    // This will be improved when layer is internalised and further improved through the
    // the user of builders and code cleanup
    let mut game = GameState {
        initialised: false,
        camera: None,
    };
    event_loop.run(|ctx, _delta_time, control, app| {
        if ctx.close_requested() {
            control.exit();
        }
        if !game.initialised() {
            game.initialise(app);
        }
        // proves the failure is only for the camera
        if ctx.is_key_pressed(KeyCode::Comma) {
            println!("Hi");
        }

        app.render().unwrap();
        app.update_camera(game.camera.as_mut().unwrap(), &ctx, _delta_time);
        app.update(ctx);
    });
}
