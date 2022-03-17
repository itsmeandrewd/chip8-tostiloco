mod null;
pub mod webgl;

pub trait Display {
    fn clear(&mut self);
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
    fn draw_pixel(&mut self, x: usize, y: usize, pixel_size: f32, turn_on: bool);
    fn get_pixel(&self, x: usize, y: usize) -> bool;
    fn initialize(&mut self);
}
