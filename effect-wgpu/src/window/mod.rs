use std::sync::Arc;

use effect_core::misc::fullscreen::FullScreenMode;
use winit::dpi::PhysicalSize;

pub struct WebWindow<'a> {
    window: Arc<winit::window::Window>,
    surface: wgpu::Surface<'a>,
    surface_config: wgpu::SurfaceConfiguration,
    mode: FullScreenMode,
}

impl<'a> WebWindow<'a> {
    pub fn new(
        window: Arc<winit::window::Window>,
        surface: wgpu::Surface<'a>,
        surface_config: wgpu::SurfaceConfiguration,
        mode: FullScreenMode,
    ) -> Self {
        Self {
            window,
            surface,
            surface_config,
            mode,
        }
    }

    pub fn resolution(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.surface_config.width, self.surface_config.height)
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn window(&self) -> &winit::window::Window {
        self.window.as_ref()
    }

    pub fn window_mut(&mut self) -> &winit::window::Window {
        &mut self.window
    }

    pub fn mode(&self) -> FullScreenMode {
        self.mode
    }
}

pub struct WebWindowSystem;

impl WebWindowSystem {
    pub fn set_resolution(window: &mut WebWindow, res: PhysicalSize<u32>, device: &wgpu::Device) {
        window.window.set_resizable(true);
        let _ = window.window.request_inner_size(res);
        window.surface_config.width = res.width;
        window.surface_config.height = res.height;
        window.surface.configure(device, &window.surface_config);
        window.window.set_resizable(false);
    }
}
