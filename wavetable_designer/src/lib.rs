use std::sync::Arc;
use nih_plug::prelude::*;
use nih_plug_egui::{create_egui_editor, EguiState, resizable_window::ResizableWindow};
use parking_lot::Mutex;

mod app_state;
mod dsp;
mod editor;

use app_state::WtState;
use egui_plot::{Line, Plot, PlotPoints};
use nih_plug_egui::egui::{self, Vec2, Color32, Stroke};

#[derive(Params)]
pub struct WtParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,

    #[id = "preview_gain"]
    pub preview_gain: FloatParam,
}

pub struct WavetableDesigner {
    params: Arc<WtParams>,
    state: Arc<Mutex<WtState>>,
    phase: f32, // audio thread phase
}

impl Default for WavetableDesigner {
    fn default() -> Self {
        Self {
            params: Arc::new(WtParams::default()),
            state: Arc::new(Mutex::new(WtState::default())),
            phase: 0.0,
        }
    }
}

impl Default for WtParams {
    fn default() -> Self {
        Self {
            editor_state: EguiState::from_size(900, 650),
            preview_gain: FloatParam::new("Preview Gain", -12.0, FloatRange::Linear { min: -60.0, max: 0.0 })
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

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> { self.params.clone() }
    
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let sample_rate = context.transport().sample_rate;
        
        // Grab values from atomic fast-path if possible, or lock
        let mut table_cache = [0.0; app_state::WT_SIZE];
        let mut playing = false;
        let mut freq = 55.0; // A1
        
        if let Some(guard) = self.state.try_lock() {
            table_cache.copy_from_slice(&guard.baked_table);
            playing = guard.preview_playing;
            freq = guard.preview_freq;
        }

        let gain = nih_plug::util::db_to_gain(self.params.preview_gain.smoothed.next());
        let phase_inc = freq / sample_rate;

        for channel_frames in buffer.iter_samples() {
            let mut sample = 0.0;
            if playing {
                let idx_f = self.phase * app_state::WT_SIZE as f32;
                let idx0 = idx_f as usize % app_state::WT_SIZE;
                let idx1 = (idx0 + 1) % app_state::WT_SIZE;
                let frac = idx_f.fract();
                
                sample = table_cache[idx0] * (1.0 - frac) + table_cache[idx1] * frac;
                
                self.phase += phase_inc;
                if self.phase >= 1.0 { self.phase -= 1.0; }
            } else {
                self.phase = 0.0;
            }

            for frame in channel_frames {
                *frame = sample * gain;
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

impl ClapPlugin for WavetableDesigner {
    const CLAP_ID: &'static str = "ai.tablestudio.wavetable_designer";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Advanced Wavetable Editor");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::Synthesizer];
}

impl Vst3Plugin for WavetableDesigner {
    const VST3_CLASS_ID: [u8; 16] = *b"TableStudioWT!!!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(WavetableDesigner);
nih_export_vst3!(WavetableDesigner);
