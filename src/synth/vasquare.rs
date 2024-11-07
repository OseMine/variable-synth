use super::WaveformGenerator;
use std::f32::consts::PI;
use rand::Rng;

pub struct VASquareWave {
    num_harmonics: usize,
    jitter: f32,
}

impl VASquareWave {
    pub fn new(num_harmonics: usize, jitter: f32) -> Self {
        Self { num_harmonics, jitter }
    }

    fn calculate_bandlimited_square(&self, phase: f32) -> f32 {
        let mut sample = 0.0;
        for n in 1..=self.num_harmonics {
            let harmonic = (2 * n - 1) as f32;
            sample += (phase * harmonic).sin() / harmonic;
        }
        sample * 4.0 / PI
    }
}

impl WaveformGenerator for VASquareWave {
    fn generate(&self, phase: f32) -> f32 {
        let jittered_phase = phase + (rand::thread_rng().gen::<f32>() - 0.5) * self.jitter * 2.0 * PI;
        let normalized_phase = jittered_phase % (2.0 * PI);
        
        self.calculate_bandlimited_square(normalized_phase)
    }
}
