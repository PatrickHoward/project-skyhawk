// TODO: Use our own mousebutton and have a way to convert sdl2::mouse::MouseButton to our native type
use sdl2::mouse::MouseButton;
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
        MouseState { left: false, right: false, middle: false }
    }
}

pub struct Mouse {
    mouse_state: MouseState,
    old_mouse_state: MouseState,
}

impl Mouse {
    pub fn new() -> Self {
        let mouse_state = MouseState::new();
        let old_mouse_state = MouseState::new();

        Mouse { mouse_state, old_mouse_state }
    }

    pub fn tick(&mut self) {
        self.old_mouse_state = self.mouse_state.clone();
    }

    pub fn set(&mut self, button: MouseButton, down: bool) {
        match button {
            MouseButton::Left => self.mouse_state.left = down,
            MouseButton::Middle => self.mouse_state.middle = down,
            MouseButton::Right => self.mouse_state.right = down,
            _ => {}
        };
    }

    pub fn is_pressed(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.mouse_state.left && !self.old_mouse_state.left,
            MouseButton::Middle => self.mouse_state.middle && !self.old_mouse_state.middle,
            MouseButton::Right => self.mouse_state.right && !self.old_mouse_state.right,
            _ => false
        }
    }

    pub fn is_down(&self, button: MouseButton) -> bool {
        match button {
            MouseButton::Left => self.mouse_state.left && self.old_mouse_state.left,
            MouseButton::Middle => self.mouse_state.middle && self.old_mouse_state.middle,
            MouseButton::Right => self.mouse_state.right && self.old_mouse_state.right,
            _ => false
        }
    }

    pub fn is_released(&self, button: MouseButton) -> bool {
        !self.is_down(button)
    }
}