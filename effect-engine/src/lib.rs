pub extern crate effect_audio as audio;
pub extern crate effect_core as core;
pub extern crate effect_events as events;
pub extern crate effect_gui as gui;
pub extern crate effect_util as util;
pub extern crate effect_vulkan as vulkan;
pub extern crate effect_wgpu as web_render;

pub mod main_loop;

use core::misc::{fullscreen::FullScreenMode, window_info::WindowInfo};

use effect_wgpu::app::effect2d::EffectWeb2D;
use main_loop::EffectEventLoopWeb;
use winit::{dpi::PhysicalSize, event_loop::EventLoop};

pub enum EngineType {
    D2,
    // D3,
}

pub enum GraphicsAPI {
    WGPU,
    // VULKAN,
}

pub enum EffectAppVariant {
    Web2D(EffectEventLoopWeb),
    // Web3D(EffectWeb3D),
}

pub struct EffectAppBuilder {
    engine_type: EngineType,
    app_name: &'static str,
    resizable_window: bool,
    graphics_api: GraphicsAPI, // pixel art should be on a per texture basis
    vsync: bool,
    fullscreen_mode: FullScreenMode,
    monitor: usize,
    resolution: PhysicalSize<u32>,
}

impl Default for EffectAppBuilder {
    fn default() -> Self {
        let engine_type = EngineType::D2;
        let app_name = "Untitled";
        let resizable_window = false;
        let graphics_api = GraphicsAPI::WGPU;
        let vsync = true;
        let fullscreen_mode = FullScreenMode::WINDOWED;
        let monitor = 0;
        let resolution = PhysicalSize::new(800, 600);
        Self {
            engine_type,
            app_name,
            resizable_window,
            graphics_api,
            vsync,
            fullscreen_mode,
            monitor,
            resolution,
        }
    }
}

#[allow(unreachable_patterns)]
impl EffectAppBuilder {
    pub fn engine_type(mut self, ty: EngineType) -> Self {
        self.engine_type = ty;
        self
    }

    pub fn app_name(mut self, app_name: &'static str) -> Self {
        self.app_name = app_name;
        self
    }

    pub fn resizable_window(mut self, resizable: bool) -> Self {
        self.resizable_window = resizable;
        self
    }

    pub fn graphics_api(mut self, graphics_api: GraphicsAPI) -> Self {
        self.graphics_api = graphics_api;
        self
    }

    pub fn vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }

    pub fn fullscreen_mode(mut self, mode: FullScreenMode) -> Self {
        self.fullscreen_mode = mode;
        self
    }

    pub fn monitor(mut self, monitor: usize) -> Self {
        self.monitor = monitor;
        self
    }

    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.resolution = PhysicalSize::new(width, height);
        self
    }

    pub fn build(self) -> EffectAppVariant {
        let window_info = WindowInfo::default()
            .app_name(self.app_name)
            .resizable(self.resizable_window)
            .fullscreen(self.fullscreen_mode)
            .monitor(self.monitor)
            .resolution(self.resolution);
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        match self.graphics_api {
            GraphicsAPI::WGPU => match self.engine_type {
                EngineType::D2 => {
                    let effect_loop = EffectEventLoopWeb::new(event_loop, window_info);
                    EffectAppVariant::Web2D(effect_loop)
                }
                _ => unimplemented!(),
            },
            _ => unimplemented!(),
        }
    }
}

#[allow(unreachable_patterns)]
impl EffectAppVariant {
    pub fn get_wgpu_2d(self) -> EffectEventLoopWeb {
        match self {
            EffectAppVariant::Web2D(val) => return val,
            _ => {
                panic!("App was not configured to use WGPU in 2D mode, please check your configuration.")
            }
        }
    }
}
