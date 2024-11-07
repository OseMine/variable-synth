mod sine;
mod saw;
mod analogsaw;
mod vasaw;
mod square;
mod analogsquare;
mod vasquare;
pub mod voice;
pub mod voice_manager;

use std::sync::Arc;
use crate::params::Waveform;

pub trait WaveformGenerator: Send + Sync {
    fn generate(&self, phase: f32) -> f32;
}

pub fn create_waveform(waveform: Waveform) -> Arc<dyn WaveformGenerator + Send + Sync> {
    match waveform {
        Waveform::Sine => Arc::new(sine::SineWave),
        Waveform::Saw => Arc::new(saw::SawWave),
        Waveform::AnalogSaw => Arc::new(analogsaw::AnalogSawWave::new(1.5, 0.6, 0.01)),
        Waveform::VASaw => Arc::new(vasaw::VASawWave::new(44100.0, 10, 0.05, 0.001)),
        Waveform::Square => Arc::new(square::SquareWave),
        Waveform::AnalogSquare => Arc::new(analogsquare::AnalogSquareWave::new(0.1)),
        Waveform::VASquare => Arc::new(vasquare::VASquareWave::new(10, 0.001)),
    }
}
