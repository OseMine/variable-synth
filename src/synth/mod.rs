mod saw;

use crate::utils::midi_note_to_freq;

pub struct Synthesizer {
    oscillator: Box<dyn Oscillator>,
    sample_rate: f32,
    current_note: Option<u8>,
}

impl Synthesizer {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            oscillator: Box::new(saw::SawOscillator::new(sample_rate)),
            sample_rate,
            current_note: None,
        }
    }

    pub fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.oscillator.set_sample_rate(sample_rate);
    }

    pub fn note_on(&mut self, note: u8, _velocity: f32) {
        let freq = midi_note_to_freq(note);
        self.oscillator.set_frequency(freq);
        self.current_note = Some(note);
    }

    pub fn note_off(&mut self, note: u8) {
        if self.current_note == Some(note) {
            self.current_note = None;
        }
    }

    pub fn generate(&mut self) -> f32 {
        if self.current_note.is_some() {
            self.oscillator.generate()
        } else {
            0.0
        }
    }

    pub fn reset(&mut self) {
        self.oscillator.reset();
        self.current_note = None;
    }
}

pub trait Oscillator: Send + Sync {
    fn new(sample_rate: f32) -> Self where Self: Sized;
    fn set_frequency(&mut self, freq: f32);
    fn set_sample_rate(&mut self, sample_rate: f32);
    fn generate(&mut self) -> f32;
    fn reset(&mut self);
}
