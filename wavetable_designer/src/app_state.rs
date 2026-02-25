use std::f32::consts::TAU;

pub const WT_SIZE: usize = 2048;
pub const DEFAULT_GRID_ROWS: usize = 8;
pub const DEFAULT_GRID_COLS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewMode {
    Off = 0,
    EditDrone = 1,
    Midi = 2,
}

impl PreviewMode {
    pub fn label(self) -> &'static str {
        match self {
            PreviewMode::Off => "Off",
            PreviewMode::EditDrone => "Edit-Drone",
            PreviewMode::Midi => "MIDI",
        }
    }
}

#[derive(Clone)]
pub struct WavetableFrame {
    pub raw: Vec<f32>,
    pub baked: Vec<f32>,
    pub is_keyframe: bool,
}

impl WavetableFrame {
    pub fn new_sine() -> Self {
        let mut raw = vec![0.0; WT_SIZE];
        for i in 0..WT_SIZE {
            raw[i] = (i as f32 / WT_SIZE as f32 * TAU).sin();
        }
        Self {
            baked: raw.clone(),
            raw,
            is_keyframe: false,
        }
    }
}

pub struct WtState {
    pub frames: Vec<WavetableFrame>,
    pub active_frame: usize,
    pub grid_rows: usize,
    pub grid_cols: usize,

    // FM Engine
    pub fm_ratio: f32,
    pub fm_amount: f32,
    pub mod_shape: usize, // 0 = Sine, 1 = Saw, 2 = Square

    // BassForge / Effects
    pub fundamental_boost: f32,
    pub wavefold_amount: f32,

    // Preview
    pub preview_mode: PreviewMode,
    pub preview_note: u8,
    pub preview_detune_cents: f32,
    pub edit_gate: bool,
}

impl WtState {
    pub fn active_frame_mut(&mut self) -> &mut WavetableFrame {
        let index = self.active_frame.min(self.frames.len().saturating_sub(1));
        self.active_frame = index;
        &mut self.frames[index]
    }

    pub fn active_frame(&self) -> &WavetableFrame {
        let index = self.active_frame.min(self.frames.len().saturating_sub(1));
        &self.frames[index]
    }
}

impl Default for WtState {
    fn default() -> Self {
        let frame_count = DEFAULT_GRID_ROWS * DEFAULT_GRID_COLS;
        let mut frames = Vec::with_capacity(frame_count);
        for _ in 0..frame_count {
            frames.push(WavetableFrame::new_sine());
        }
        frames[0].is_keyframe = true;

        Self {
            frames,
            active_frame: 0,
            grid_rows: DEFAULT_GRID_ROWS,
            grid_cols: DEFAULT_GRID_COLS,
            fm_ratio: 2.0,
            fm_amount: 0.0,
            mod_shape: 0,
            fundamental_boost: 0.0,
            wavefold_amount: 0.0,
            preview_mode: PreviewMode::Off,
            preview_note: 33,
            preview_detune_cents: 0.0,
            edit_gate: false,
        }
    }
}
