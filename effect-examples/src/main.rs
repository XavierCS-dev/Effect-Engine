use effect_engine::events::main_loop::EffectEventLoop;
use effect_engine::EffectAppBuilder;

fn main() {
    let (mut app, event_loop) = EffectAppBuilder::default().build().get_wgpu_2d();
    let layers = Vec::new();
    let camera = app.init_camera(45.0);
    app.resize_window(800, 600);
    EffectEventLoop::run(event_loop, |ctx, _delta_time, control| {
        if ctx.close_requested() {
            control.exit();
        }
        app.render(&layers, &camera).unwrap();
        app.update(ctx);
    });
}
