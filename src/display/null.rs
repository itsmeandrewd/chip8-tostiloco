use crate::Display;

pub struct NullDisplay {
    pub cleared: bool
}

impl Default for NullDisplay {
    fn default() -> Self {
        Self {
            cleared: false
        }
    }
}

impl Display for NullDisplay {
    fn clear(&mut self) {
        self.cleared = true;
    }

    fn get_width(&self) -> usize {
        1
    }

    fn get_height(&self) -> usize {
        1
    }

    fn draw_pixel(&mut self, _x: usize, _y: usize, _pixel_size: f32, _turn_on: bool) {}

    fn get_pixel(&self, _x: usize, _y: usize) -> bool {
        true
    }

    fn initialize(&mut self) {}
}
