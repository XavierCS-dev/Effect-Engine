use std::collections::HashSet;
pub mod camera2d;
use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

// Should provide all the currently pressed keys
// and the keys released on the previous frame
pub struct EffectEvent {
    keys_pressed: HashSet<PhysicalKey>,
    keys_released: HashSet<PhysicalKey>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,
    mouse_within_window: bool,
    mouse_position: PhysicalPosition<f64>,
    mouse_travel: (f64, f64),
    window_resized: bool,
    window_size: winit::dpi::PhysicalSize<u32>,
    scale_factor_changed: bool,
    scale_factor: f64,
    close_requested: bool,
}

impl EffectEvent {
    pub fn new() -> Self {
        let keys_pressed = HashSet::new();
        let keys_released = HashSet::new();
        let mouse_pressed = HashSet::new();
        let mouse_released = HashSet::new();
        let mouse_within_window = false;
        let mouse_position = PhysicalPosition::new(0.0, 0.0);
        let mouse_travel = (0.0, 0.0);
        let window_resized = false;
        let scale_factor_changed = false;
        let window_size = winit::dpi::PhysicalSize::new(0, 0);
        let close_requested = false;
        let scale_factor = 1.0;
        Self {
            keys_pressed,
            keys_released,
            mouse_pressed,
            mouse_released,
            mouse_within_window,
            mouse_position,
            mouse_travel,
            window_resized,
            window_size,
            scale_factor_changed,
            close_requested,
            scale_factor,
        }
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.keys_pressed.contains(&PhysicalKey::Code(key))
    }

    pub fn is_key_released(&self, key: KeyCode) -> bool {
        self.keys_released.contains(&PhysicalKey::Code(key))
    }

    pub fn is_mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_pressed.contains(&button)
    }

    pub fn is_mouse_released(&self, button: MouseButton) -> bool {
        self.mouse_released.contains(&button)
    }

    pub fn mouse_position(&self) -> PhysicalPosition<f64> {
        self.mouse_position
    }

    pub fn mouse_delta(&self) -> (f64, f64) {
        self.mouse_travel
    }

    pub fn window_resized(&self) -> bool {
        self.window_resized
    }

    pub fn window_size(&self) -> winit::dpi::PhysicalSize<u32> {
        self.window_size
    }

    pub fn scale_factor_changed(&self) -> bool {
        self.scale_factor_changed
    }

    pub fn close_requested(&self) -> bool {
        self.close_requested
    }

    pub fn scale_factor(&self) -> f64 {
        self.scale_factor
    }
}

pub struct EffectEventSystem;
impl EffectEventSystem {
    pub fn force_resize(context: &mut EffectEvent) {
        context.window_resized = true;
    }

    pub fn device_event_update(context: &mut EffectEvent, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                context.mouse_travel = *delta;
            }
            _ => (),
        };
    }
    pub fn window_event_update(context: &mut EffectEvent, event: &WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                context.close_requested = true;
            }
            WindowEvent::Resized(size) => {
                context.window_resized = true;
                context.window_size = *size;
            }
            WindowEvent::ScaleFactorChanged {
                scale_factor,
                inner_size_writer,
            } => {
                context.scale_factor_changed = true;
                context.scale_factor = *scale_factor
            }
            WindowEvent::KeyboardInput { event, .. } => match event.state {
                ElementState::Pressed => {
                    context.keys_pressed.insert(event.physical_key);
                }
                ElementState::Released => {
                    match context.keys_pressed.take(&event.physical_key) {
                        Some(key) => {
                            context.keys_released.insert(key);
                        }
                        _ => (),
                    };
                }
            },
            WindowEvent::MouseInput { state, button, .. } => match state {
                ElementState::Pressed => {
                    context.mouse_pressed.insert(*button);
                }
                ElementState::Released => match context.mouse_pressed.take(button) {
                    Some(button) => {
                        context.mouse_released.insert(button);
                    }
                    _ => (),
                },
            },
            WindowEvent::CursorMoved { position, .. } => {
                context.mouse_position = *position;
            }
            WindowEvent::CursorEntered { .. } => {
                context.mouse_within_window = true;
            }
            WindowEvent::CursorLeft { .. } => {
                context.mouse_within_window = false;
            }
            _ => (),
        };
    }

    pub fn clear_released(context: &mut EffectEvent) {
        context.keys_released.clear();
        context.mouse_released.clear();
    }

    pub fn reset_window_changes(context: &mut EffectEvent) {
        context.window_resized = false;
        context.scale_factor_changed = false;
    }
}
