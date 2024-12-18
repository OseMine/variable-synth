use nih_plug::prelude::*;
use std::sync::Arc;

#[derive(Params)]
pub struct VariableSynthParams {
    #[id = "waveform"]
    pub waveform: EnumParam<Waveform>,
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "tuning"]
    pub tuning: FloatParam,  // New tuning parameter
}

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum Waveform {
    #[name = "Sine"]
    Sine,
    #[name = "Saw"]
    Saw,
    #[name = "Square"]
    Square,
    #[name = "Analog Saw"]
    AnalogSaw,
    #[name = "VA Saw"]
    VASaw,
    #[name = "Analog Square"]
    AnalogSquare,
    #[name = "VA Square"]
    VASquare,
    #[name = "Vintage Saw"]
    VintageSaw,
    
}

impl Default for VariableSynthParams {
    fn default() -> Self {
        Self {
            waveform: EnumParam::new("Waveform", Waveform::Sine)
                .with_callback(Arc::new(|_| ())),
            gain: FloatParam::new(
                "Gain",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit(" dB"),
            tuning: FloatParam::new(
                "Tuning",
                440.0, // Default value: 440 Hz
                FloatRange::Linear { min: 20.0, max: 880.0 }, // Tuning range from 20 Hz to 880 Hz
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_unit("Hz"),
        }
    }
}

impl VariableSynthParams {
    pub fn update_waveform(&self) -> bool {
        self.waveform.value() != self.waveform.default_plain_value()
    }
}
