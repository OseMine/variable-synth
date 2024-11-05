use nih_plug::prelude::*;

#[derive(Params)]
pub struct VariableSynthParams {
    #[id = "waveform"]
    pub waveform: EnumParam<Waveform>,
    #[id = "gain"]
    pub gain: FloatParam,
}

#[derive(Enum, PartialEq)]
pub enum Waveform {
    #[name = "Sine"]
    Sine,
    #[name = "Saw"]
    Saw,
}

impl Default for VariableSynthParams {
    fn default() -> Self {
        Self {
            waveform: EnumParam::new("Waveform", Waveform::Sine),
            gain: FloatParam::new(
                "Gain",
                0.5,
                FloatRange::Linear { min: 0.0, max: 1.0 },
            ),
        }
    }
}
