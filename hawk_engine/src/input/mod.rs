mod keyboard;
mod mouse;

use crate::input::keyboard::Keyboard;
use crate::input::mouse::Mouse;

use sdl2::mouse::MouseButton;

pub enum InputMapping {
    Keyboard(i32),
    Mouse(MouseButton),
}

pub struct Input {
    keyboard: Keyboard,
    mouse: Mouse,
    //gamepad: Gamepad,
}

impl Input {
    pub fn new() -> Self {
        let keyboard = Keyboard::new();
        let mouse = Mouse::new();
        Input { keyboard, mouse}
    }

    // This is an open/closed principle violation, maybe we can make a nice abstraction
    // using traits?
    pub fn mapping_pressed(&self, mapping: InputMapping) -> bool {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.is_pressed(scancode),
            InputMapping::Mouse(button) => self.mouse.is_pressed(button),
        }
    }

    pub fn mapping_down(&self, mapping: InputMapping) -> bool {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.is_down(scancode),
            InputMapping::Mouse(button) => self.mouse.is_down(button),
        }
    }

    pub fn mapping_released(&self, mapping: InputMapping) -> bool {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.is_released(scancode),
            InputMapping::Mouse(button) => self.mouse.is_released(button),
        }
    }

    pub fn set(&mut self, mapping: InputMapping, down: bool) {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.set(scancode, down),
            InputMapping::Mouse(button) => self.mouse.set(button, down),
        }
    }

    pub fn tick(&mut self) {
        self.keyboard.tick();
    }
}