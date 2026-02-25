use std::f32::consts::TAU;

/// Number of samples in a single wavetable frame (one complete waveform cycle).
pub const WT_SIZE: usize = 2048;
/// Default number of rows in the frame selection grid.
pub const DEFAULT_GRID_ROWS: usize = 8;
/// Default number of columns in the frame selection grid.
pub const DEFAULT_GRID_COLS: usize = 8;

/// Controls which audio preview mode the plugin uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreviewMode {
    /// No audio output; the phase accumulator is held at zero.
    Off = 0,
    /// Plays the selected note at the chosen frequency while the user is
    /// actively dragging on the waveform canvas (gated by `WtState::edit_gate`).
    EditDrone = 1,
    /// Monophonic MIDI input — the last received note-on wins.
    /// Velocity controls the output amplitude.
    Midi = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MorphMode {
    Crossfade,
    Spectral,
    SpectralZeroPhase,
}

impl PreviewMode {
    /// Returns the human-readable label used in the UI combo-box.
    pub fn label(self) -> &'static str {
        match self {
            PreviewMode::Off => "Off",
            PreviewMode::EditDrone => "Edit-Drone",
            PreviewMode::Midi => "MIDI",
        }
    }
}

/// A single wavetable frame storing both the user-drawn raw samples and the
/// processed (baked) output samples.
///
/// Both `raw` and `baked` always contain exactly [`WT_SIZE`] `f32` samples in
/// the range `[-1.0, 1.0]`.
#[derive(Clone)]
pub struct WavetableFrame {
    /// Samples as drawn by the user (before the FM / effects chain).
    pub raw: Vec<f32>,
    /// Samples after the bake pipeline (FM stacking → fundamental boost →
    /// wavefold → normalisation).  This is what the audio engine reads.
    pub baked: Vec<f32>,
    /// Whether this frame is marked as a keyframe in future interpolation work.
    pub is_keyframe: bool,
}

impl WavetableFrame {
    /// Creates a new frame pre-populated with a full-amplitude sine wave.
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

/// All mutable state shared between the audio thread and the egui editor.
///
/// This struct is wrapped in `Arc<Mutex<WtState>>` so that the audio `process`
/// callback and the UI closure can each hold a reference.  The audio thread
/// uses `try_lock` to avoid blocking; if the editor holds the lock, the
/// process callback outputs silence for that buffer.
pub struct WtState {
    /// All wavetable frames.  Length is always `grid_rows × grid_cols`.
    pub frames: Vec<WavetableFrame>,
    /// Index of the frame currently selected for editing and audio output.
    pub active_frame: usize,
    /// Number of rows in the frame selection grid.
    pub grid_rows: usize,
    /// Number of columns in the frame selection grid.
    pub grid_cols: usize,

    // FM Engine
    /// Modulator-to-carrier frequency ratio for the 2-op FM engine.
    pub fm_ratio: f32,
    /// FM modulation depth (0 = off, higher values = more modulation).
    pub fm_amount: f32,
    /// Modulator waveform shape: 0 = Sine, 1 = Saw, 2 = Square.
    pub mod_shape: usize,

    // Spectral Morph
    pub spectral_morph_amount: f32,
    pub spectral_formant: f32,
    pub spectral_smear: f32,
    pub spectral_stretch: f32,
    pub spectral_warp: f32,

    // BassForge / Effects
    /// Amount of additive fundamental sine to mix in (0 = off, 2 = double).
    pub fundamental_boost: f32,
    /// Wavefolding intensity applied after the FM stage (0 = off, 1 = full).
    pub wavefold_amount: f32,

    // Preview
    /// Current audio preview mode.
    pub preview_mode: PreviewMode,
    /// MIDI note number used for Edit-Drone preview (0–127).
    pub preview_note: u8,
    /// Fine-tune offset for the Edit-Drone note, in cents (±50 ¢).
    pub preview_detune_cents: f32,
    /// Set to `true` while the user is dragging on the canvas; gates the
    /// Edit-Drone preview output.
    pub edit_gate: bool,
}

impl WtState {
    /// Returns a mutable reference to the currently active frame.
    ///
    /// Clamps `active_frame` to a valid index if the frame list is shorter than
    /// expected (e.g. after a grid resize).
    pub fn active_frame_mut(&mut self) -> &mut WavetableFrame {
        let index = self.active_frame.min(self.frames.len().saturating_sub(1));
        self.active_frame = index;
        &mut self.frames[index]
    }

    /// Returns a shared reference to the currently active frame.
    ///
    /// Clamps `active_frame` to a valid index if the frame list is shorter than
    /// expected.
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
            spectral_morph_amount: 0.35,
            spectral_formant: 0.0,
            spectral_smear: 0.0,
            spectral_stretch: 0.0,
            spectral_warp: 0.0,
            fundamental_boost: 0.0,
            wavefold_amount: 0.0,
            preview_mode: PreviewMode::Off,
            preview_note: 33,
            preview_detune_cents: 0.0,
            edit_gate: false,
        }
    }
}
