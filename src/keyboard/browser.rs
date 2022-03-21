use crate::keyboard::Keyboard;

#[derive(Default)]
pub struct BrowserKeyboard {
    key_pressed: u8,
}

impl Keyboard for BrowserKeyboard {
    fn initialize(&mut self) {
        /*let document = web_sys::window().unwrap().document().unwrap();
        {
            let key_down_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
                self.set_key(event.key_code() as u8);
            }) as Box<dyn FnMut(_)>);
            document
                .add_event_listener_with_callback(
                    "keydown",
                    key_down_closure.as_ref().unchecked_ref(),
                )
                .unwrap();
            key_down_closure.forget();
        }*/
    }

    fn set_key(&mut self, key: u8) {
        match key {
            // 1 to 9
            49 => self.key_pressed = 0x1,
            50 => self.key_pressed = 0x2,
            51 => self.key_pressed = 0x3,
            52 => self.key_pressed = 0x4,
            53 => self.key_pressed = 0x5,
            54 => self.key_pressed = 0x6,
            55 => self.key_pressed = 0x7,
            56 => self.key_pressed = 0x8,
            57 => self.key_pressed = 0x9,

            // a to f
            65 => self.key_pressed = 0xa,
            66 => self.key_pressed = 0xb,
            67 => self.key_pressed = 0xc,
            68 => self.key_pressed = 0xd,
            69 => self.key_pressed = 0xe,
            70 => self.key_pressed = 0xf,

            // used to clear the key pressed
            _ => self.key_pressed = 0x0,
        }
    }

    fn get_key(&mut self) -> u8 {
        self.key_pressed
    }
}
