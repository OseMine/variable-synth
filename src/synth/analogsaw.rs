use super::WaveformGenerator;
use std::f32::consts::PI;
use rand::Rng;

pub struct AnalogSawWave {
    sharpness: f32,
    asymmetry: f32,
    jitter: f32,
}

impl AnalogSawWave {
    pub fn new(sharpness: f32, asymmetry: f32, jitter: f32) -> Self {
        Self {
            sharpness,
            asymmetry,
            jitter,
        }
    }
}

impl WaveformGenerator for AnalogSawWave {
    fn generate(&self, phase: f32) -> f32 {
        let normalized_phase = phase / (2.0 * PI);
        
        // Fügt Jitter hinzu, um den Start und das Maximum weniger perfekt zu machen
        let jittered_phase = normalized_phase + (rand::random::<f32>() - 0.5) * self.jitter;
        
        // Erzeugt eine asymmetrische Sägezahnwelle
        let asymmetric_saw = if jittered_phase < self.asymmetry {
            jittered_phase / self.asymmetry
        } else {
            (jittered_phase - self.asymmetry) / (1.0 - self.asymmetry) - 1.0
        };
        
        // Wendet eine exponentielle Funktion an, um den Abgang zu gestalten
        let shaped_saw = if asymmetric_saw >= 0.0 {
            asymmetric_saw.powf(self.sharpness)
        } else {
            -((-asymmetric_saw).powf(self.sharpness))
        };
        
        // Skaliert das Ergebnis auf den Bereich [-1, 1]
        shaped_saw * 2.0 - 1.0
    }
}
