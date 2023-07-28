
use super::*;

pub struct Input {
    pressed: KeyCode,
    previous: KeyCode,
}

impl Input {
    pub(crate) fn update(&mut self, keycode: KeyCode) {
        self.previous = self.pressed;
        self.pressed = keycode
    }

    pub fn is_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed == keycode
    }

    pub fn just_pressed(&self, keycode: KeyCode) -> bool {
        self.pressed == keycode && self.previous != keycode
    }

    pub fn just_released(&self, keycode: KeyCode) -> bool {
        self.pressed == keycode && self.previous != keycode
    }
}

impl CoreId for Input {
    fn id() -> usize {
        config::RES_LIMIT - 1
    }
}

impl Default for Input {
    fn default() -> Self {
        Self { 
            pressed: KeyCode::RAlt,
            previous: KeyCode::RAlt,
        }
    }
}