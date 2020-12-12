// TODO: Use our own mousebutton and have a way to convert sdl2::mouse::MouseButton to our native type
use sdl2::mouse::MouseButton;

use crate::math::Vec2f32;

// pub enum MouseButton {
//     Left,
//     Right,
//     Middle,
//     ScrollUp,
//     ScrollDown,
// }

#[derive(Clone)]
struct MouseState {
    pub left: bool,
    pub right: bool,
    pub middle: bool,
}

impl MouseState {
    pub fn new() -> Self {
        MouseState {
            left: false,
            right: false,
            middle: false,
        }
    }
}

type MousePos = Vec2f32;

pub struct Mouse {
    mouse_state: MouseState,
    old_mouse_state: MouseState,

    mouse_pos: MousePos,
    old_mouse_pos: MousePos,
}

impl Mouse {
    pub fn new() -> Self {
        let mouse_state = MouseState::new();
        let old_mouse_state = MouseState::new();

        let mouse_pos = MousePos::new(0.0, 0.0);
        let old_mouse_pos = MousePos::new(400.0, 300.0); // TODO: I shouldn't be set here!

        Mouse {
            mouse_state,
            old_mouse_state,

            mouse_pos,
            old_mouse_pos,
        }
    }

    pub fn tick(&mut self) {
        self.old_mouse_state = self.mouse_state.clone();
        self.old_mouse_pos = self.mouse_pos.clone();
    }

    pub fn set(&mut self, button: MouseButton, down: bool) {
        match button {
            MouseButton::Left => self.mouse_state.left = down,
            MouseButton::Middle => self.mouse_state.middle = down,
            MouseButton::Right => self.mouse_state.right = down,
            _ => {}
        };
    }

    pub fn set_mousepos(&mut self, pos: MousePos) {
        self.mouse_pos = pos;
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.mouse_state.left && !self.old_mouse_state.left,
            MouseButton::Middle => self.mouse_state.middle && !self.old_mouse_state.middle,
            MouseButton::Right => self.mouse_state.right && !self.old_mouse_state.right,
            _ => false,
        }
    }

    pub fn is_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.mouse_state.left && self.old_mouse_state.left,
            MouseButton::Middle => self.mouse_state.middle && self.old_mouse_state.middle,
            MouseButton::Right => self.mouse_state.right && self.old_mouse_state.right,
            _ => false,
        }
    }

    pub fn is_released(&self, button: MouseButton) -> bool {
        !self.is_down(button)
    }

    pub fn mousepos(&self) -> MousePos {
        self.mouse_pos
    }

    pub fn mousepos_offset_since_last_frame(&self) -> MousePos {
        let offset_x = self.mouse_pos.x - self.old_mouse_pos.x;
        let offset_y = self.mouse_pos.y - self.old_mouse_pos.y;

        // TODO: Move sensitivity out of this function
        const SENSITIVITY: f32 = 0.1;
        MousePos::new(offset_x, offset_y).mul_scalar(SENSITIVITY)
    }
}
