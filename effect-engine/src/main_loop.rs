use std::time::{Duration, Instant};

use effect_core::misc::fullscreen::FullScreenMode;
use effect_core::misc::window_info::WindowInfo;
use effect_events::input::EffectEvent;
use effect_events::input::EffectEventSystem;
use web_render::app::effect2d::EffectWeb2D;
use web_render::engine::engine2d::WebEngine2D;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;
use winit::monitor::VideoModeHandle;

pub struct EffectLoopWeb<F>
where
    F: FnMut(&mut EffectEvent, Duration, &ActiveEventLoop, &mut EffectWeb2D) -> (),
{
    user_loop: F,
    event: EffectEvent,
    time_before: Instant,
    time_after: Instant,
    window_info: WindowInfo,
    app: Option<EffectWeb2D>,
}

impl<F> ApplicationHandler<()> for EffectLoopWeb<F>
where
    F: FnMut(&mut EffectEvent, Duration, &ActiveEventLoop, &mut EffectWeb2D) -> (),
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let attributes = winit::window::Window::default_attributes()
            .with_title(self.window_info.name)
            .with_inner_size(self.window_info.dimensions)
            .with_resizable(self.window_info.resizable);
        let window = event_loop
            .create_window(attributes)
            .expect("Could not create window");

        let mut monitors: Vec<MonitorHandle> = window.available_monitors().collect();
        if self.window_info.monitor as usize >= monitors.len() {
            panic!("Could not find monitor {}", self.window_info.monitor);
        }
        let monitor = monitors.remove(self.window_info.monitor);
        let mut video_modes: Vec<VideoModeHandle> = monitor.video_modes().collect();
        window.set_fullscreen(match self.window_info.fullscreen {
            FullScreenMode::WINDOWED => None,
            FullScreenMode::BORDERLESS => {
                Some(winit::window::Fullscreen::Borderless(Some(monitor)))
            }
            FullScreenMode::FULLSCREEN => Some(winit::window::Fullscreen::Exclusive(
                video_modes
                    .pop()
                    .expect("Monitor does not support any video modes"),
            )),
        });
        let engine = pollster::block_on(WebEngine2D::new(window, self.window_info.vsync));
        self.app = Some(EffectWeb2D::new(engine));
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        EffectEventSystem::window_event_update(&mut self.event, &event);
    }

    fn user_event(&mut self, event_loop: &winit::event_loop::ActiveEventLoop, event: ()) {}

    fn device_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        EffectEventSystem::device_event_update(&mut self.event, &event);
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.time_after = Instant::now();
        (self.user_loop)(
            &mut self.event,
            self.time_after - self.time_before,
            event_loop,
            unsafe { &mut self.app.as_mut().unwrap_unchecked() },
        );
        self.time_before = self.time_after;
        EffectEventSystem::clear_released(&mut self.event);
    }
}

pub struct EffectEventLoopWeb {
    event_loop: EventLoop<()>,
    window_info: WindowInfo,
}

impl EffectEventLoopWeb {
    pub fn new(event_loop: EventLoop<()>, window_info: WindowInfo) -> Self {
        Self {
            event_loop,
            window_info,
        }
    }

    pub fn run<F>(self, user_loop: F)
    where
        F: FnMut(&mut EffectEvent, Duration, &ActiveEventLoop, &mut EffectWeb2D) -> (),
    {
        let event = EffectEvent::new();
        let time_before = Instant::now();
        let time_after = Instant::now();
        let window_info = self.window_info;
        let app = None;

        let mut effect_loop = EffectLoopWeb {
            user_loop,
            event,
            time_before,
            time_after,
            window_info,
            app,
        };

        let _ = self.event_loop.run_app(&mut effect_loop);
    }
}
