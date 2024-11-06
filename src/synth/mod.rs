mod sine;
mod saw;
mod analogsaw;

use std::sync::Arc;
use crate::params::Waveform;

pub trait WaveformGenerator {
    fn generate(&self, phase: f32) -> f32;
}

pub fn create_waveform(waveform: Waveform) -> Arc<dyn WaveformGenerator + Send + Sync> {
    match waveform {
        Waveform::Sine => Arc::new(sine::SineWave),
        Waveform::Saw => Arc::new(saw::SawWave),
        Waveform::AnalogSaw => Arc::new(analogsaw::AnalogSawWave::new(1.5, 0.6, 0.01)),

    }
}

