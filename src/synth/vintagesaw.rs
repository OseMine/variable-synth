use super::WaveformGenerator;
use std::f32::consts::E;

pub struct VintageSaw {
    
}

impl VintageSaw {
    // Konstruktor, der die Struktur ohne T und b erstellt
    pub fn new() -> Self {
        VintageSaw {}
    }

    // Automatische Berechnung von T und b basierend auf der Phase
    fn calculate_parameters(&self, phase: f32) -> (f32, f32) {
        // Beispielberechnung für T und b
        // T könnte von der Phase abhängen, um die Wellenform zu steuern
        let t = 1.0 / phase.max(0.01); // Verhindert Division durch Null, sicherer Wert für T

        // b könnte in Relation zur Phase berechnet werden
        let b = 5.0 * phase.max(0.01); // Skalierung von b je nach Phase

        (t, b)
    }
}

impl WaveformGenerator for VintageSaw {
    fn generate(&self, phase: f32) -> f32 {
        // Berechne T und b aus der aktuellen Phase
        let (T, b) = self.calculate_parameters(phase);

        // Berechne den Parameter a basierend auf T und b
        let a = -2.0 / (E.powf(-b * T) - 1.0);

        // Berechne die Exponentialwellenform
        a * E.powf(-b * phase) + (1.0 - a)
    }
}
