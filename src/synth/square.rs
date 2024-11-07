use super::WaveformGenerator;
use std::f32::consts::PI;

pub struct SquareWave;

impl WaveformGenerator for SquareWave {
    fn generate(&self, phase: f32) -> f32 {
        if phase < PI {
            1.0
        } else {
            -1.0
        }
    }
}
