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
}

impl Default for EffectAppBuilder {
    fn default() -> Self {
        let engine_type = EngineType::D2;
        let app_name = "";
        let window_dimensions = PhysicalSize::new(800, 600);
        let resizable_window = false;
        let graphics_api = GraphicsAPI::WGPU;
        let vsync = true;
        Self {
            engine_type,
            app_name,
            window_dimensions,
            resizable_window,
            graphics_api,
            vsync,
        }
    }
}

impl EffectAppBuilder {
    pub fn engine_type(mut self, ty: EngineType, vari: EffectAppVariant) -> Self {
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

    pub fn build(self) -> EffectAppVariant {
        match self.graphics_api {
            GraphicsAPI::WGPU => match self.engine_type {
                EngineType::D2 => EffectAppVariant::Web2D(EffectWeb2D::new(
                    self.window_dimensions,
                    45.0,
                    self.vsync,
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
