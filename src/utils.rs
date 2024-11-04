use std::f32::consts::PI;

pub fn hz_to_radians(freq: f32, sample_rate: f32) -> f32 {
    2.0 * PI * freq / sample_rate
}

pub fn midi_note_to_freq(note: u8) -> f32 {
    440.0 * 2.0_f32.powf((note as f32 - 69.0) / 12.0)
}
