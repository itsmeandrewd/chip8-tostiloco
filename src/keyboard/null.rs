use crate::keyboard::Keyboard;

#[derive(Default)]
pub struct NullKeyboard {
    key_pressed: u8,
}

impl Keyboard for NullKeyboard {
    fn initialize(&mut self) {
        self.key_pressed = 0;
    }

    fn set_key(&mut self, key: u8) {
        self.key_pressed = key;
    }

    fn get_key(&mut self) -> u8 {
        self.key_pressed
    }
}
