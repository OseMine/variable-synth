use super::WaveformGenerator;
use std::sync::Arc;

pub struct Voice {
    waveform_generator: Arc<dyn WaveformGenerator + Send + Sync>,
    phase: f32,
    frequency: f32,
    is_active: bool,
}

impl Voice {
    pub fn new(waveform_generator: Arc<dyn WaveformGenerator + Send + Sync>) -> Self {
        Self {
            waveform_generator,
            phase: 0.0,
            frequency: 440.0,
            is_active: false,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn get_frequency(&self) -> f32 {
        self.frequency
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn generate_sample(&mut self, sample_rate: f32) -> f32 {
        if !self.is_active {
            return 0.0;
        }

        let sample = self.waveform_generator.generate(self.phase);
        self.phase += self.frequency * 2.0 * std::f32::consts::PI / sample_rate;
        if self.phase > 2.0 * std::f32::consts::PI {
            self.phase -= 2.0 * std::f32::consts::PI;
        }
        sample
    }
}
