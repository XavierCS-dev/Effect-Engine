use std::collections::{HashMap, HashSet};

use winit::{
    dpi::PhysicalPosition,
    event::{DeviceEvent, ElementState, Event, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

// Should provide all the currently pressed keys
// and the keys released on the previous frame
pub struct Context2D {
    keys_pressed: HashSet<PhysicalKey>,
    keys_released: HashSet<PhysicalKey>,
    mouse_pressed: HashSet<MouseButton>,
    mouse_released: HashSet<MouseButton>,
    mouse_within_window: bool,
    mouse_position: PhysicalPosition<f64>,
    mouse_travel: (f64, f64),
}

impl Context2D {
    pub fn new() -> Self {
        let keys_pressed = HashSet::new();
        let keys_released = HashSet::new();
        let mouse_pressed = HashSet::new();
        let mouse_released = HashSet::new();
        let mouse_within_window = false;
        let mouse_position = PhysicalPosition::new(0.0, 0.0);
        let mouse_travel = (0.0, 0.0);
        Self {
            keys_pressed,
            keys_released,
            mouse_pressed,
            mouse_released,
            mouse_within_window,
            mouse_position,
            mouse_travel,
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
}

pub struct Context2DSystem;
impl Context2DSystem {
    pub fn update(context: &mut Context2D, event: &Event<()>) {
        match event {
            Event::WindowEvent { window_id, event } => match event {
                WindowEvent::KeyboardInput {
                    device_id,
                    event,
                    is_synthetic,
                } => match event.state {
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
                WindowEvent::MouseInput {
                    device_id,
                    state,
                    button,
                } => match state {
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
                WindowEvent::CursorMoved {
                    device_id,
                    position,
                } => {
                    context.mouse_position = *position;
                }
                WindowEvent::CursorEntered { device_id } => {
                    context.mouse_within_window = true;
                }
                WindowEvent::CursorLeft { device_id } => {
                    context.mouse_within_window = false;
                }
                _ => (),
            },
            Event::DeviceEvent { device_id, event } => match event {
                DeviceEvent::MouseMotion { delta } => {
                    context.mouse_travel = *delta;
                }
                _ => (),
            },
            _ => (),
        }
    }

    pub fn clear_released(context: &mut Context2D) {
        context.keys_released.clear();
        context.mouse_released.clear();
    }
}
