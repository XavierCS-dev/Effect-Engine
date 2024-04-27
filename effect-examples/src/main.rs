use effect_engine::core::id::{LayerID, TextureID};
use effect_engine::core::primitives::vector::Vector3;
use effect_engine::events::main_loop::EffectEventLoop;
use effect_engine::web_render::entity::entity2d::WebEntity2D;
use effect_engine::web_render::layer::WebLayer2D;
use effect_engine::web_render::texture::texture2d::{WebTexture2D, WebTexture2DSystem};
use effect_engine::EffectAppBuilder;
use winit::dpi::PhysicalSize;

fn main() {
    let (mut app, event_loop) = EffectAppBuilder::default()
        .resizable_window(true)
        .build()
        .get_wgpu_2d();
    let tex_id = TextureID("Tree");
    let texture = WebTexture2D::new(tex_id, "assets/tree.png");
    let tex = vec![texture];
    let mut layer = app
        .init_layer(LayerID(0), tex, PhysicalSize::new(32, 32), true)
        .unwrap();
    let ent = WebEntity2D::new(Vector3::new(0.0, 0.0, -1.0), &layer, tex_id);
    let ents = vec![&ent];
    app.set_entities(&mut layer, &ents);
    let layers = vec![layer];
    EffectEventLoop::run(event_loop, |ctx, _delta_time, control| {
        if ctx.close_requested() {
            control.exit();
        }
        app.render(&layers).unwrap();
        app.update(ctx, &mut None);
    });
}
