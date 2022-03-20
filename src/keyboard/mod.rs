pub mod browser;
pub mod null;

pub trait Keyboard {
    fn initialize(&mut self);
    fn set_key(&mut self, key: u8);
    fn get_key(&mut self) -> u8;
}
