use crate::Display;

pub struct NullDisplay {
    pub cleared: bool,
    pub vram: [u8; 64 * 32]
}

impl Default for NullDisplay {
    fn default() -> Self {
        Self {
            cleared: false,
            vram: [0; 64 * 32]
        }
    }
}

impl Display for NullDisplay {
    fn clear(&mut self) {
        self.cleared = true;
        self.vram = [0; 64 * 32];
    }

    fn get_width(&self) -> usize {
        64
    }

    fn get_height(&self) -> usize {
        32
    }

    fn draw_pixel(&mut self, x: usize, y: usize, _pixel_size: f32, turn_on: bool) {
        if turn_on {
            self.vram[y * self.get_width() + x] = 1;
        } else {
            self.vram[y * self.get_width() + x] = 0;
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> bool {
        self.vram[y * self.get_width() + x] == 1
    }

    fn initialize(&mut self) {}
}
