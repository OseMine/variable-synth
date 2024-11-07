mod sine;
mod saw;
mod analogsaw;
mod vasaw;

use std::sync::Arc;
use crate::params::Waveform;
use vasaw::VASawWave;

pub trait WaveformGenerator {
    fn generate(&self, phase: f32) -> f32;
}

pub fn create_waveform(waveform: Waveform) -> Arc<dyn WaveformGenerator + Send + Sync> {
    match waveform {
        Waveform::Sine => Arc::new(sine::SineWave),
        Waveform::Saw => Arc::new(saw::SawWave),
        Waveform::AnalogSaw => Arc::new(analogsaw::AnalogSawWave::new(1.5, 0.6, 0.01)),
        Waveform::VASaw => Arc::new(VASawWave::new(44100.0, 10, 0.05, 0.001)),
    }
}

