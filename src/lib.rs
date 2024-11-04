use nih_plug::prelude::*;
use std::sync::Arc;

mod params;
mod synth;
mod utils;

use params::VariableSynthParams;
use synth::Synthesizer;

struct VariableSynth {
    params: Arc<VariableSynthParams>,
    synth: Synthesizer,
}

impl Default for VariableSynth {
    fn default() -> Self {
        Self {
            params: Arc::new(VariableSynthParams::default()),
            synth: Synthesizer::new(44100.0),
        }
    }
}

impl Plugin for VariableSynth {
    const NAME: &'static str = "Variable Synth";
    const VENDOR: &'static str = "OseMine";
    const URL: &'static str = "https://github.com/OseMine/variable-synth";
    const EMAIL: &'static str = "info@example.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: None,
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        }
    ];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.synth.set_sample_rate(buffer_config.sample_rate);
        true
    }

    fn reset(&mut self) {
        self.synth.reset();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        self.synth.note_on(note, velocity);
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        self.synth.note_off(note);
                    }
                    _ => (),
                }

                next_event = context.next_event();
            }

            let output = self.synth.generate();
            for sample in channel_samples {
                *sample = output;
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for VariableSynth {
    const CLAP_ID: &'static str = "com.muzikar.variable-synth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A variable synthesizer plugin");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for VariableSynth {
    const VST3_CLASS_ID: [u8; 16] = *b"VariableSynth123";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(VariableSynth);
nih_export_vst3!(VariableSynth);
