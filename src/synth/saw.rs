use super::WaveformGenerator;
use std::f32::consts::PI;

pub struct SawWave;

impl WaveformGenerator for SawWave {
    fn generate(&self, phase: f32) -> f32 {
        2.0 * (phase / (2.0 * PI)).fract() - 1.0
    }
}
