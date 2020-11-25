type KeyboardState = [bool; 264];

pub struct Keyboard {
    key_state: KeyboardState,
    old_key_state: KeyboardState,
}

impl Keyboard {
    pub fn new() -> Self {
        let key_state = [false; 264];
        let old_key_state = [false; 264];

        Keyboard {
            key_state, old_key_state
        }
    }

    pub fn tick(&mut self) {
        self.old_key_state = self.key_state.clone();
    }

    pub fn set(&mut self, scancode: i32, down: bool) {
        self.key_state[scancode as usize] = down;
    }

    pub fn is_pressed(&self, scancode: i32) -> bool {
        self.key_state[scancode as usize] && !self.old_key_state[scancode as usize]
    }

    pub fn is_down(&self, scancode: i32) -> bool {
        self.key_state[scancode as usize] && self.old_key_state[scancode as usize]
    }

    pub fn is_released(&self, scancode: i32) -> bool {
        !self.is_down(scancode)
    }
}