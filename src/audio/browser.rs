use crate::audio::AudioSource;
use web_sys::{AudioContext, OscillatorNode, OscillatorType};

pub struct BrowserAudioSource {
    context: AudioContext,
    current_oscillator: Option<OscillatorNode>,
    is_playing: bool,
}

impl Default for BrowserAudioSource {
    fn default() -> Self {
        // todo: delay this to later since browsers don't allow audiocontexts created prior
        // to user action
        let context = AudioContext::new().unwrap();
        Self {
            context,
            current_oscillator: None,
            is_playing: false,
        }
    }
}

impl BrowserAudioSource {
    fn play_oscillator(&mut self) {
        let oscillator = self.context.create_oscillator().unwrap();
        oscillator.connect_with_audio_node(&self.context.destination()).unwrap();
        oscillator.set_type(OscillatorType::Sine);
        oscillator.frequency().set_value(440.0);
        oscillator.start().unwrap();

        self.current_oscillator = Some(oscillator);
    }
}

impl AudioSource for BrowserAudioSource {
    fn initialize(&mut self) {
        self.context = AudioContext::new().unwrap();
    }

    fn start_sound(&mut self) {
        if !self.is_playing {
            self.play_oscillator();
        }
        self.is_playing = true;
    }

    fn stop_sound(&mut self) {
        if self.is_playing {
            self.current_oscillator.as_ref().unwrap().stop().unwrap();
        }
        self.is_playing = false;
    }
}
