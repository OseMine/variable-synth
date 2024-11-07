use nih_plug::prelude::*;
use std::sync::Arc;
mod params;
mod utils;
mod synth;
use params::VariableSynthParams;
use synth::{create_waveform, voice_manager::VoiceManager};

struct VariableSynth {
    params: Arc<VariableSynthParams>,
    voice_manager: VoiceManager,
}

impl Default for VariableSynth {
    fn default() -> Self {
        let params = Arc::new(VariableSynthParams::default());
        let waveform_generator = create_waveform(params.waveform.value());
        Self {
            params: params.clone(),
            voice_manager: VoiceManager::new(waveform_generator),
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
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        // Konvertiere velocity von f32 zu u8
                        let velocity_u8 = (velocity * 127.0) as u8;
                        self.voice_manager.note_on(note, velocity_u8);
                    }
                    NoteEvent::NoteOff { note, .. } => {
                        self.voice_manager.note_off(note);
                    }
                    _ => (),
                }
                
                next_event = context.next_event();
            }
    
            // Überprüfe, ob sich die Wellenform geändert hat
            if self.params.waveform.value() != self.params.waveform.default_plain_value() {
                let new_waveform_generator = create_waveform(self.params.waveform.value());
                self.voice_manager = VoiceManager::new(new_waveform_generator);
            }
    
            let gain = self.params.gain.smoothed.next();
            let tuning_factor = self.params.tuning.value() / 440.0;
    
            // Erzeuge einen temporären Buffer, der die generierten Samples speichert
            let mut temp_buffer = vec![0.0; channel_samples.len()];
            self.voice_manager.generate_samples(&mut temp_buffer, sample_rate);
    
            // Berechne die endgültigen Samples und schreibe sie in den Ausgabe-Buffer
            for (temp_sample, output_sample) in temp_buffer.iter().zip(channel_samples) {
                *output_sample = *temp_sample * gain * tuning_factor;
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
