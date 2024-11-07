use super::WaveformGenerator;
use std::f32::consts::PI;

pub struct AnalogSquareWave {
    transition_time: f32,
}

impl AnalogSquareWave {
    pub fn new(transition_time: f32) -> Self {
        Self { transition_time }
    }
}

impl WaveformGenerator for AnalogSquareWave {
    fn generate(&self, phase: f32) -> f32 {
        let normalized_phase = phase / (2.0 * PI);
        let transition_width = self.transition_time / 2.0;

        if normalized_phase < 0.5 - transition_width {
            1.0
        } else if normalized_phase < 0.5 + transition_width {
            1.0 - (normalized_phase - (0.5 - transition_width)) / self.transition_time * 2.0
        } else if normalized_phase < 1.0 - transition_width {
            -1.0
        } else {
            -1.0 + (normalized_phase - (1.0 - transition_width)) / self.transition_time * 2.0
        }
    }
}
