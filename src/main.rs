use effect_engine;

fn main() {
    println!("Hello, world!");
    let (app, event_loop) = effect_engine::init_engine();
    let mut before = Instant::now();
    let mut after = Instant::now();
    let _ = event_loop.run(|event, control| {
        after = Instant::now();
        let delta_time = after - before;
        self.engine.input(&event, &delta_time);
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                control.exit();
            }
            Event::AboutToWait => {
                self.engine.update(&delta_time);
                // Should probably handle this somewhere..
                self.engine.render().unwrap();
            }
            _ => (),
        }
        before = after;
    });
}
