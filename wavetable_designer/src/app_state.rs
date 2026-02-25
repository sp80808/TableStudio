use std::f32::consts::TAU;

pub const WT_SIZE: usize = 2048;

pub struct WtState {
    pub raw_table: Vec<f32>,
    pub baked_table: Vec<f32>,
    
    // FM Engine
    pub fm_ratio: f32,
    pub fm_amount: f32,
    pub mod_shape: usize, // 0 = Sine, 1 = Saw, 2 = Square
    
    // BassForge / Effects
    pub fundamental_boost: f32,
    pub wavefold_amount: f32,
    
    // Preview
    pub preview_playing: bool,
    pub preview_freq: f32,
}

impl Default for WtState {
    fn default() -> Self {
        let mut raw = vec![0.0; WT_SIZE];
        for i in 0..WT_SIZE {
            raw[i] = (i as f32 / WT_SIZE as f32 * TAU).sin();
        }
        Self {
            raw_table: raw.clone(),
            baked_table: raw,
            fm_ratio: 2.0,
            fm_amount: 0.0,
            mod_shape: 0,
            fundamental_boost: 0.0,
            wavefold_amount: 0.0,
            preview_playing: false,
            preview_freq: 55.0, // A1 for bass
        }
    }
}
