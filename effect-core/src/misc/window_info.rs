use winit::dpi::PhysicalSize;

use super::fullscreen::FullScreenMode;

#[derive(Clone, Copy)]
pub struct WindowInfo {
    pub name: &'static str,
    pub resizable: bool,
    pub fullscreen: FullScreenMode,
    pub monitor: usize,
    pub vsync: bool,
    pub resolution: PhysicalSize<u32>,
}

impl WindowInfo {
    pub fn app_name(mut self, app_name: &'static str) -> Self {
        self.name = app_name;
        self
    }

    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn fullscreen(mut self, mode: FullScreenMode) -> Self {
        self.fullscreen = mode;
        self
    }

    pub fn monitor(mut self, index: usize) -> Self {
        self.monitor = index;
        self
    }

    pub fn resolution(mut self, resolution: PhysicalSize<u32>) -> Self {
        self.resolution = resolution;
        self
    }
}

impl Default for WindowInfo {
    fn default() -> Self {
        let dimensions = PhysicalSize::new(800, 600);
        let name = "Untitled";
        let resizable = true;
        let fullscreen = FullScreenMode::WINDOWED;
        let monitor = 0;
        let vsync = true;
        let resolution = PhysicalSize::new(800, 600);
        Self {
            name,
            resizable,
            fullscreen,
            monitor,
            vsync,
            resolution,
        }
    }
}
