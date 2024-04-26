## Effect Engine
Effect Engine 2, a Rust-based game enginedesigned for simplicity and high performance.

NOTE: This game engine is still in alpha state, the API is very likely to change for 
even patch versions, many basic features are missing and performance isn't where it should be.
Documentation is also currently subpar, with unnecessary dependencies.

To get started, include the latest version of this package in your Cargo.toml.
The following example is for v0.2.6-alpha and likely will need to be modified.
Concrete examples will be provided in the main repository, these should always work.

```Rust
use effect_engine::events::main_loop::EffectEventLoop;
use effect_engine::EffectAppBuilder;

fn main() {
    let (mut app, event_loop) = EffectAppBuilder::default()
        .build()
        .get_wgpu_2d();
    let layers = Vec::new();
    let camera = app.init_camera(45.0);
    EffectEventLoop::run(event_loop, |ctx, _delta_time, _control| {
        app.render(&layers, &camera).unwrap();
        app.update(ctx);
    });
}
```

A Vulkan renderer is also planned for the future.
