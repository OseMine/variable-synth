use super::WaveformGenerator;
use std::f32::consts::PI;
use rand::Rng;

pub struct VASawWave {
    phase: f32,
    sample_rate: f32,
    num_harmonics: usize,
    dc_offset: f32,
    jitter: f32,
}

impl VASawWave {
    pub fn new(sample_rate: f32, num_harmonics: usize, dc_offset: f32, jitter: f32) -> Self {
        Self {
            phase: 0.0,
            sample_rate,
            num_harmonics,
            dc_offset,
            jitter,
        }
    }

    fn calculate_bandlimited_saw(&self, phase: f32) -> f32 {
        let mut sample = 0.0;
        for n in 1..=self.num_harmonics {
            let harmonic = n as f32;
            sample += (-1.0_f32).powi(n as i32 + 1) * (phase * harmonic).sin() / harmonic;
        }
        sample * 2.0 / PI
    }
}

impl WaveformGenerator for VASawWave {
    fn generate(&self, phase: f32) -> f32 {
        let jittered_phase = phase + (rand::thread_rng().gen::<f32>() - 0.5) * self.jitter * 2.0 * PI;
        let normalized_phase = jittered_phase % (2.0 * PI);
        
        let bandlimited_saw = self.calculate_bandlimited_saw(normalized_phase);
        
        // Fügen Sie den DC-Offset hinzu und normalisieren Sie die Ausgabe
        (bandlimited_saw + self.dc_offset) / (1.0 + self.dc_offset.abs())
    }
}
