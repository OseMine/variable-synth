use nih_plug::prelude::*;
use std::sync::Arc;
#[derive(Params)]
pub struct VariableSynthParams {
    #[id = "waveform"]
    pub waveform: EnumParam<Waveform>,
    #[id = "gain"]
    pub gain: FloatParam,
}

#[derive(Enum, PartialEq, Clone, Copy)]
pub enum Waveform {
    #[name = "Sine"]
    Sine,
    #[name = "Saw"]
    Saw,
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
        }
    }
}

impl VariableSynthParams {
    pub fn update_waveform(&self) -> bool {
        self.waveform.value() != self.waveform.default_plain_value()
    }
}