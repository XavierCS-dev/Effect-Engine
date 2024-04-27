use effect_engine::core::id::{LayerID, TextureID};
use effect_engine::core::misc::fullscreen::FullScreenMode;
use effect_engine::core::primitives::vector::Vector3;
use effect_engine::events::main_loop::EffectEventLoop;
use effect_engine::web_render::entity::entity2d::WebEntity2D;
use effect_engine::web_render::layer::WebLayer2D;
use effect_engine::web_render::texture::texture2d::{WebTexture2D, WebTexture2DSystem};
use effect_engine::EffectAppBuilder;
use winit::dpi::PhysicalSize;

fn main() {
    let (mut app, event_loop) = EffectAppBuilder::default()
        .resizable_window(false)
        .fullscreen_mode(FullScreenMode::BORDERLESS)
        .monitor(0)
        .build()
        .get_wgpu_2d();

    // Extremely verbose just to get a texture on screen.
    // This will be improved when layer is internalised and further improved through the
    // the user of builders and code cleanup
    let tex_id = TextureID("Tree");
    let texture = WebTexture2D::new(tex_id, "assets/tree.png");
    let tex = vec![texture];
    app.init_layer(LayerID(0), tex, PhysicalSize::new(32, 32), true)
        .unwrap();
    let ent = app.init_entity(Vector3::new(0.0, 0.0, -1.0), LayerID(0), tex_id);
    let ents = vec![&ent];
    app.set_entities(LayerID(0), &ents);
    EffectEventLoop::run(event_loop, |ctx, _delta_time, control| {
        if ctx.close_requested() {
            control.exit();
        }
        app.render().unwrap();
        app.update(ctx, &mut None);
    });
}
