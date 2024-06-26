pub extern crate effect_audio as audio;
pub extern crate effect_core as core;
pub extern crate effect_events as events;
pub extern crate effect_gui as gui;
pub extern crate effect_util as util;
pub extern crate effect_vulkan as vulkan;
pub extern crate effect_wgpu as web_render;

use core::misc::fullscreen::FullScreenMode;

use effect_wgpu::app::effect2d::EffectWeb2D;
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
    Web2D((EffectWeb2D, EventLoop<()>)),
    // Web3D(EffectWeb3D),
}

pub struct EffectAppBuilder {
    engine_type: EngineType,
    app_name: &'static str,
    window_dimensions: PhysicalSize<u32>,
    resizable_window: bool,
    graphics_api: GraphicsAPI, // pixel art should be on a per texture basis
    vsync: bool,
    fullscreen_mode: FullScreenMode,
    monitor: u32,
}

impl Default for EffectAppBuilder {
    fn default() -> Self {
        let engine_type = EngineType::D2;
        let app_name = "Untitled";
        let window_dimensions = PhysicalSize::new(800, 600);
        let resizable_window = false;
        let graphics_api = GraphicsAPI::WGPU;
        let vsync = true;
        let fullscreen_mode = FullScreenMode::WINDOWED;
        let monitor = 0;
        Self {
            engine_type,
            app_name,
            window_dimensions,
            resizable_window,
            graphics_api,
            vsync,
            fullscreen_mode,
            monitor,
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

    pub fn window_dimensions(mut self, width: u32, height: u32) -> Self {
        self.window_dimensions = PhysicalSize::new(width, height);
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

    pub fn monitor(mut self, monitor: u32) -> Self {
        self.monitor = monitor;
        self
    }

    pub fn build(self) -> EffectAppVariant {
        match self.graphics_api {
            GraphicsAPI::WGPU => match self.engine_type {
                EngineType::D2 => EffectAppVariant::Web2D(EffectWeb2D::new(
                    self.window_dimensions,
                    self.vsync,
                    self.app_name,
                    self.resizable_window,
                    self.fullscreen_mode,
                    self.monitor,
                )),
                _ => {
                    unimplemented!()
                }
            },
            _ => {
                unimplemented!()
            }
        }
    }
}

#[allow(unreachable_patterns)]
impl EffectAppVariant {
    pub fn get_wgpu_2d(self) -> (EffectWeb2D, EventLoop<()>) {
        match self {
            EffectAppVariant::Web2D(val) => return val,
            _ => {
                panic!("App was not configured to use WGPU in 2D mode, please check your configuration.")
            }
        }
    }
}
