use crate::audio::AudioSource;
use web_sys::{AudioContext, OscillatorNode, OscillatorType};

pub struct BrowserAudioSource {
    oscillator: OscillatorNode,
    is_playing: bool
}

impl Default for BrowserAudioSource {
    fn default() -> Self {
        // todo: delay this to later since browsers don't allow audiocontexts created prior
        // to user action
        let context = AudioContext::new().unwrap();
        let oscillator = context.create_oscillator().unwrap();
        oscillator.connect_with_audio_node(&context.destination()).unwrap();

        Self {
            oscillator,
            is_playing: false
        }
    }
}

impl AudioSource for BrowserAudioSource {
    fn initialize(&mut self) {
        let context = AudioContext::new().unwrap();
        let oscillator = context.create_oscillator().unwrap();
        oscillator.connect_with_audio_node(&context.destination()).unwrap();

        self.oscillator.set_type(OscillatorType::Sine);
        self.oscillator.frequency().set_value(440.0);
    }

    fn start_sound(&mut self) {
        if !self.is_playing {
            self.oscillator.start().unwrap();
        }
        self.is_playing = true;
    }

    fn stop_sound(&mut self) {
        self.oscillator.stop().unwrap();
        self.is_playing = false;
    }
}