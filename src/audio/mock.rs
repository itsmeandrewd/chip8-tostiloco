use crate::audio::AudioSource;

#[derive(Default)]
pub struct MockAudioSource {
    pub is_playing: bool
}

impl AudioSource for MockAudioSource {
    fn initialize(&mut self) {
        self.is_playing = false;
    }

    fn start_sound(&mut self) {
        self.is_playing = true;
    }

    fn stop_sound(&mut self) {
        self.is_playing = false;
    }
}