use effect_engine::events::main_loop::EffectEventLoop;
use effect_engine::EffectAppBuilder;

fn main() {
    let (mut app, event_loop) = EffectAppBuilder::default()
        .app_name("Test")
        .window_dimensions(600, 600)
        .graphics_api(effect_engine::GraphicsAPI::WGPU)
        .engine_type(effect_engine::EngineType::D2)
        .resizable_window(false)
        .build()
        .get_wgpu_2d();
    let layers = Vec::new();
    let camera = app.init_camera(45.0);
    app.resize_window(800, 600);
    EffectEventLoop::run(event_loop, |ctx, _delta_time, _control| {
        app.render(&layers, &camera).unwrap();
        app.update(ctx);
    });
}
