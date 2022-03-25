pub mod browser;
pub mod mock;

pub trait AudioSource {
    fn initialize(&mut self);
    fn start_sound(&mut self);
    fn stop_sound(&mut self);
}
