use std::sync::Arc;

use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, EguiState};
use parking_lot::Mutex;

mod app_state;
mod dsp;
mod editor;
pub mod widgets;
mod widgets;

use app_state::{PreviewMode, WtState};
use dsp::note_to_freq;

/// NIH-plug parameter set for the Wavetable Designer plugin.
///
/// Persists `editor_state` (window size/position) and exposes a single
/// `preview_gain` float parameter controlling the output level of all preview
/// modes.
#[derive(Params)]
pub struct WtParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

    #[id = "preview_gain"]
    pub preview_gain: FloatParam,
}

/// Top-level plugin struct.
///
/// Owns the shared [`WtState`] (wrapped in an `Arc<Mutex>` so the egui editor
/// and the audio thread can both access it) plus per-audio-thread state for the
/// phase accumulator and MIDI voice tracking.
pub struct WavetableDesigner {
    params: Arc<WtParams>,
    state: Arc<Mutex<WtState>>,
    phase: f32,
    sample_rate: f32,

    midi_note_id: u8,
    midi_note_freq: f32,
    midi_note_gain: Smoother<f32>,
    edit_gate_gain: Smoother<f32>,
}

impl Default for WavetableDesigner {
    fn default() -> Self {
        Self {
            params: Arc::new(WtParams::default()),
            state: Arc::new(Mutex::new(WtState::default())),
            phase: 0.0,
            sample_rate: 44_100.0,
            midi_note_id: 0,
            midi_note_freq: 1.0,
            midi_note_gain: Smoother::new(SmoothingStyle::Linear(5.0)),
            edit_gate_gain: Smoother::new(SmoothingStyle::Linear(5.0)),
        }
    }
}

impl Default for WtParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(980, 700),
            preview_gain: FloatParam::new(
                "Preview Gain",
                -12.0,
                FloatRange::Linear { min: -60.0, max: 0.0 },
            )
            .with_unit("dB"),
        }
    }
}

impl Plugin for WavetableDesigner {
    const NAME: &'static str = "TableStudio Wavetable Designer";
    const VENDOR: &'static str = "Antigravity";
    const URL: &'static str = "https://tablestudio.ai";
    const EMAIL: &'static str = "info@tablestudio.ai";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: None,
        main_output_channels: std::num::NonZeroU32::new(2),
        aux_input_ports: &[],
        aux_output_ports: &[],
        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;

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
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    fn reset(&mut self) {
        self.phase = 0.0;
        self.midi_note_id = 0;
        self.midi_note_freq = 1.0;
        self.midi_note_gain.reset(0.0);
        self.edit_gate_gain.reset(0.0);
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let mut next_event = context.next_event();

        let mut table_cache = [0.0; app_state::WT_SIZE];
        let (preview_mode, preview_note, preview_detune, edit_gate) = {
            if let Some(guard) = self.state.try_lock() {
                table_cache.copy_from_slice(&guard.active_frame().baked);
                (
                    guard.preview_mode,
                    guard.preview_note,
                    guard.preview_detune_cents,
                    guard.edit_gate,
                )
            } else {
                // If the editor is holding the lock, we can't access the state.
                // In this case, we'll just output silence.
                (PreviewMode::Off, 0, 0.0, false)
            }
        };

        let gain = util::db_to_gain(self.params.preview_gain.smoothed.next());
        if preview_mode == PreviewMode::EditDrone {
            let target = if edit_gate { 1.0 } else { 0.0 };
            self.edit_gate_gain.set_target(self.sample_rate, target);
        }

        for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
            // MIDI handling for monophonic mode
            while let Some(event) = next_event {
                if event.timing() > sample_id as u32 {
                    break;
                }

                match event {
                    NoteEvent::NoteOn { note, velocity, .. } => {
                        if velocity > 0.0 {
                            self.midi_note_id = note;
                            self.midi_note_freq = util::midi_note_to_freq(note);
                            self.midi_note_gain.set_target(self.sample_rate, velocity);
                        } else if note == self.midi_note_id {
                            self.midi_note_gain.set_target(self.sample_rate, 0.0);
                        }
                    }
                    NoteEvent::NoteOff { note, .. } if note == self.midi_note_id => {
                        self.midi_note_gain.set_target(self.sample_rate, 0.0);
                    }
                    NoteEvent::PolyPressure { note, pressure, .. } if note == self.midi_note_id => {
                        self.midi_note_gain.set_target(self.sample_rate, pressure);
                    }
                    _ => {}
                }

                next_event = context.next_event();
            }

            let sample = match preview_mode {
                PreviewMode::Off => {
                    self.phase = 0.0;
                    0.0
                }
                PreviewMode::EditDrone => {
                    let freq = note_to_freq(preview_note, preview_detune);
                    sample_from_table(
                        &mut self.phase,
                        freq,
                        self.sample_rate,
                        &table_cache,
                    ) * self.edit_gate_gain.next()
                }
                PreviewMode::Midi => {
                    sample_from_table(
                        &mut self.phase,
                        self.midi_note_freq,
                        self.sample_rate,
                        &table_cache,
                    ) * self.midi_note_gain.next()
                }
            } * gain;

            for frame in channel_samples {
                *frame = sample;
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        let state = self.state.clone();
        let params = self.params.clone();
        let egui_state = params.editor_state.clone();

        create_egui_editor(
            egui_state.clone(),
            (),
            |_, _| {},
            move |ctx, setter, _state| {
                editor::draw_ui(ctx, setter, &state, &params);
            },
        )
    }
}

/// Advance `phase` by one sample and return the linearly-interpolated value
/// from `table` at the current phase position.
///
/// `phase` is maintained in the range `[0, 1)`.  The table is assumed to
/// contain exactly [`app_state::WT_SIZE`] samples representing one full cycle.
fn sample_from_table(phase: &mut f32, freq: f32, sample_rate: f32, table: &[f32]) -> f32 {
    let phase_inc = freq / sample_rate;
    let idx_f = *phase * app_state::WT_SIZE as f32;
    let idx0 = idx_f as usize % app_state::WT_SIZE;
    let idx1 = (idx0 + 1) % app_state::WT_SIZE;
    let frac = idx_f.fract();
    let sample = table[idx0] * (1.0 - frac) + table[idx1] * frac;

    *phase += phase_inc;
    if *phase >= 1.0 {
        *phase -= 1.0;
    }

    sample
}

impl ClapPlugin for WavetableDesigner {
    const CLAP_ID: &'static str = "ai.tablestudio.wavetable_designer";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Advanced Wavetable Editor");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Synthesizer];
}

impl Vst3Plugin for WavetableDesigner {
    const VST3_CLASS_ID: [u8; 16] = *b"TableStudioWT!!!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(WavetableDesigner);
nih_export_vst3!(WavetableDesigner);
