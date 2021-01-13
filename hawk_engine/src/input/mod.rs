mod keyboard;
mod mouse;

use crate::{
    input::{keyboard::Keyboard, mouse::Mouse},
    math::Vec2,
};

use sdl2::mouse::MouseButton;

pub enum InputMapping {
    Keyboard(i32),
    Mouse(MouseButton),
    MousePos(i32, i32), // TODO: I shouldn't live here. A separate "axis" enum should exist instead.
    MouseScroll(i32),
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

        Input { keyboard, mouse }
    }

    // This is an open/closed principle violation, maybe we can make a nice abstraction
    // using traits?
    pub fn mapping_pressed(&self, mapping: InputMapping) -> bool {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.is_pressed(scancode),
            InputMapping::Mouse(button) => self.mouse.is_pressed(button),
            _ => false,
        }
    }

    pub fn mapping_down(&self, mapping: InputMapping) -> bool {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.is_down(scancode),
            InputMapping::Mouse(button) => self.mouse.is_down(button),
            _ => false,
        }
    }

    pub fn mapping_released(&self, mapping: InputMapping) -> bool {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.is_released(scancode),
            InputMapping::Mouse(button) => self.mouse.is_released(button),
            _ => false,
        }
    }

    pub fn get_mousepos(&self) -> Vec2 {
        self.mouse.mousepos()
    }

    pub fn get_mousepos_offset(&self) -> Vec2 {
        self.mouse.mousepos_offset_since_last_tick()
    }

    pub fn get_mouse_scroll_delta(&self) -> f32 {
        self.mouse.scroll_y_delta()
    }

    pub fn set(&mut self, mapping: InputMapping, down: bool) {
        match mapping {
            InputMapping::Keyboard(scancode) => self.keyboard.set(scancode, down),
            InputMapping::Mouse(button) => self.mouse.set(button, down),
            InputMapping::MousePos(x, y) => self.mouse.set_mousepos(Vec2::new(x as f32, y as f32)),
            InputMapping::MouseScroll(y) => {
                self.mouse.set_scrollwheel(y as f32);
            }
        }
    }

    pub fn tick(&mut self) {
        self.keyboard.tick();
        self.mouse.tick();
    }
}
