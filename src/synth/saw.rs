use super::Oscillator;
use crate::utils::hz_to_radians;

pub struct SawOscillator {
    phase: f32,
    phase_delta: f32,
    sample_rate: f32,
    frequency: f32,
}

impl Oscillator for SawOscillator {
    fn new(sample_rate: f32) -> Self {
        Self {
            phase: 0.0,
            phase_delta: 0.0,
            sample_rate,
            frequency: 440.0,
        }
    }

    fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
        self.update_phase_delta();
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate;
        self.update_phase_delta();
    }

    fn generate(&mut self) -> f32 {
        let output = 2.0 * self.phase - 1.0;
        self.phase += self.phase_delta;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        output
    }

    fn reset(&mut self) {
        self.phase = 0.0;
    }
}

impl SawOscillator {
    fn update_phase_delta(&mut self) {
        self.phase_delta = hz_to_radians(self.frequency, self.sample_rate) / (2.0 * std::f32::consts::PI);
    }
}
