use effect_engine::EffectAppBuilder;
use effect_events::main_loop::EffectEventLoop;

fn main() {
    let (mut app, event_loop) = EffectAppBuilder::default()
        .app_name("Test")
        .window_dimensions(600, 600)
        .graphics_api(effect_engine::GraphicsAPI::WGPU)
        .build()
        .get_wgpu_2d();
    let layers = Vec::new();
    let camera = app.init_camera(45.0);
    EffectEventLoop::run(event_loop, |ctx, delta_time, control| {
        app.render(&layers, &camera).unwrap();
    });
}
