use nih_plug::prelude::*;
use std::sync::Arc;

#[derive(Params)]
pub struct VariableSynthParams {
    #[id = "synth_type"]
    pub synth_type: EnumParam<SynthType>,

    #[id = "gain"]
    pub gain: FloatParam,

    #[id = "attack"]
    pub attack: FloatParam,

    #[id = "decay"]
    pub decay: FloatParam,

    #[id = "sustain"]
    pub sustain: FloatParam,

    #[id = "release"]
    pub release: FloatParam,
}

#[derive(Enum, PartialEq)]
pub enum SynthType {
    #[name = "Saw"]
    Saw,
    #[name = "Sine"]
    Sine,
    // Fügen Sie hier weitere Synthesizer-Typen hinzu
}

impl Default for VariableSynthParams {
    fn default() -> Self {
        Self {
            synth_type: EnumParam::new("Synth Type", SynthType::Saw)
                .with_callback(Arc::new(|_| ())),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            attack: FloatParam::new(
                "Attack",
                0.01,
                FloatRange::Skewed {
                    min: 0.001,
                    max: 1.0,
                    factor: FloatRange::skew_factor(-2.0),
                },
            )
            .with_unit(" s")
            .with_value_to_string(formatters::v2s_f32_rounded(3)),

            decay: FloatParam::new(
                "Decay",
                0.1,
                FloatRange::Skewed {
                    min: 0.001,
                    max: 2.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_unit(" s")
            .with_value_to_string(formatters::v2s_f32_rounded(3)),

            sustain: FloatParam::new(
                "Sustain",
                1.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 1.0,
                },
            )
            .with_value_to_string(formatters::v2s_f32_percentage(2)),

            release: FloatParam::new(
                "Release",
                0.1,
                FloatRange::Skewed {
                    min: 0.001,
                    max: 5.0,
                    factor: FloatRange::skew_factor(-1.0),
                },
            )
            .with_unit(" s")
            .with_value_to_string(formatters::v2s_f32_rounded(3)),
        }
    }
}

impl VariableSynthParams {
    pub fn update_synth(&self, synth: &mut crate::synth::Synthesizer) {
        // Implementierung hier
        let _ = synth; // Um die Warnung zu vermeiden
    }
    
}
