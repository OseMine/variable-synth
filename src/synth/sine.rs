use super::WaveformGenerator;

pub struct SineWave;

impl WaveformGenerator for SineWave {
    fn generate(&self, phase: f32) -> f32 {
        phase.sin()
    }
}
