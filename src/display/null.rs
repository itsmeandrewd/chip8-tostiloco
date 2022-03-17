use crate::Display;

struct NullDisplay {}

impl Display for NullDisplay {
    fn clear(&mut self) {}

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
