use nih_plug::prelude::*;
use std::sync::Arc;
mod params;
mod utils;
mod synth;
use params::VariableSynthParams;
use utils::midi_note_to_freq;
use synth::{WaveformGenerator, create_waveform};
extern crate rand;

struct VariableSynth {
    params: Arc<VariableSynthParams>,
    current_note: Option<u8>,
    phase: f32,
    waveform_generator: Arc<dyn WaveformGenerator + Send + Sync>,
}

impl Default for VariableSynth {
    fn default() -> Self {
        let params = Arc::new(VariableSynthParams::default());
        Self {
            waveform_generator: create_waveform(params.waveform.value()),
            params,
            current_note: None,
            phase: 0.0,
        }
    }
}

impl Plugin for VariableSynth {
    const NAME: &'static str = "Variable Synth";
    const VENDOR: &'static str = "Muzikar";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();
        let sample_rate = context.transport().sample_rate as f32;

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            while let Some(event) = next_event {
                if event.timing() > sample_id as u32 {
                    break;
                }
                match event {
                    NoteEvent::NoteOn { note, .. } => {
                        self.current_note = Some(note);
                        self.phase = 0.0;
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        if self.current_note == Some(note) {
                            self.current_note = None;
                        }
                    }
                    _ => (),
                }
                next_event = context.next_event();
            }

            // Überprüfe, ob sich die Wellenform geändert hat
            if self.params.waveform.value() != self.params.waveform.default_plain_value() {
                self.waveform_generator = create_waveform(self.params.waveform.value());
            }

            if let Some(note) = self.current_note {
                let base_frequency = midi_note_to_freq(note); // Get frequency from MIDI note
                let tuning_factor = self.params.tuning.value(); // Get tuning factor from the parameter
                let frequency = base_frequency * (tuning_factor / 440.0); // Adjust frequency by tuning factor

                self.phase += frequency * 2.0 * std::f32::consts::PI / sample_rate;
                if self.phase > 2.0 * std::f32::consts::PI {
                    self.phase -= 2.0 * std::f32::consts::PI;
                }

                // Use the waveform generator to calculate the sample
                let sample = self.waveform_generator.generate(self.phase);

                let gain = self.params.gain.smoothed.next();
                let output = sample * gain;

                for sample in channel_samples {
                    *sample = output;
                }
            } else {
                for sample in channel_samples {
                    *sample = 0.0;
                }
            }
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for VariableSynth {
    const CLAP_ID: &'static str = "de.muzikar.variablesynth";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A variable waveform synthesizer");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,
        ClapFeature::Synthesizer,
        ClapFeature::Stereo,
    ];
}

impl Vst3Plugin for VariableSynth {
    const VST3_CLASS_ID: [u8; 16] = *b"VariableSynth...";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(VariableSynth);
nih_export_vst3!(VariableSynth);
