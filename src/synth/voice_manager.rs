use super::voice::Voice;
use super::WaveformGenerator;
use std::sync::Arc;
// Importiere die Utils-Funktionen
use crate::utils::{hz_to_radians, midi_note_to_freq};

const MAX_VOICES: usize = 16;

pub struct VoiceManager {
    voices: Vec<Voice>,
}

impl VoiceManager {
    pub fn new(waveform_generator: Arc<dyn WaveformGenerator + Send + Sync>) -> Self {
        let mut voices = Vec::with_capacity(MAX_VOICES);
        for _ in 0..MAX_VOICES {
            voices.push(Voice::new(waveform_generator.clone()));
        }
        Self { voices }
    }

    pub fn note_on(&mut self, note: u8, _velocity: u8) {
        let frequency = midi_note_to_freq(note); // Verwende die Utils-Funktion
        if let Some(voice) = self.voices.iter_mut().find(|v| !v.is_active()) {
            voice.set_frequency(frequency);
            voice.set_active(true);
        }
    }

    pub fn note_off(&mut self, note: u8) {
        let frequency = midi_note_to_freq(note); // Verwende die Utils-Funktion
        if let Some(voice) = self.voices.iter_mut().find(|v| v.is_active() && (v.get_frequency() - frequency).abs() < 0.01) {
            voice.set_active(false);
        }
    }

    pub fn generate_samples(&mut self, buffer: &mut [f32], sample_rate: f32) {
        for sample in buffer.iter_mut() {
            *sample = 0.0;
            for voice in &mut self.voices {
                *sample += voice.generate_sample(sample_rate);
            }
            *sample /= MAX_VOICES as f32;
        }
    }
}
