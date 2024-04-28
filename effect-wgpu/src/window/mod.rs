use std::sync::Arc;

use effect_core::misc::fullscreen::FullScreenMode;
use winit::dpi::PhysicalSize;

pub struct WebWindow {
    window: Arc<winit::window::Window>,
    dimensions: PhysicalSize<u32>,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    mode: FullScreenMode,
    always_resizable: bool,
}

impl WebWindow {
    pub fn new(
        window: Arc<winit::window::Window>,
        dimensions: PhysicalSize<u32>,
        surface: wgpu::Surface,
        surface_config: wgpu::SurfaceConfiguration,
        mode: FullScreenMode,
        always_resizable: bool,
    ) -> Self {
        Self {
            window,
            dimensions,
            surface,
            surface_config,
            mode,
            always_resizable,
        }
    }

    pub fn dimensions(&self) -> PhysicalSize<u32> {
        self.dimensions
    }

    pub fn resolution(&self) -> PhysicalSize<u32> {
        PhysicalSize::new(self.surface_config.width, self.surface_config.height)
    }

    pub fn surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn window(&self) -> &Arc<winit::window::Window> {
        &self.window
    }

    pub fn window_mut(&mut self) -> &winit::window::Window {
        &mut self.window
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
        if !window.always_resizable {
            window.window.set_resizable(false);
        }
    }
}
