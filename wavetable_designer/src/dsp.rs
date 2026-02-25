use crate::app_state::{WtState, WT_SIZE};
use std::f32::consts::TAU;

pub fn bake_wavetable(state: &mut WtState) {
    let mut baked = state.raw_table.clone();
    
    // 1. FM Stacking (2-op)
    if state.fm_amount > 0.001 {
        let mut fm_table = vec![0.0; WT_SIZE];
        for i in 0..WT_SIZE {
            // Modulator phase
            let mod_phase = (i as f32 / WT_SIZE as f32 * state.fm_ratio * TAU) % TAU;
            let mod_out = match state.mod_shape {
                0 => mod_phase.sin(),
                1 => std::f32::consts::FRAC_1_PI * (mod_phase - std::f32::consts::PI), // saw
                2 => if mod_phase < std::f32::consts::PI { 1.0 } else { -1.0 },       // square
                _ => 0.0,
            };
            
            // Carrier phase (modulated)
            let base_phase = i as f32 / WT_SIZE as f32;
            let carrier_phase = (base_phase + (mod_out * state.fm_amount / TAU)).fract();
            let mut c_idx = (carrier_phase * WT_SIZE as f32) as usize;
            if c_idx >= WT_SIZE { c_idx = WT_SIZE - 1; }
            fm_table[i] = state.raw_table[c_idx];
        }
        baked = fm_table;
    }
    
    // 2. Fundamental Boost (BassForge)
    if state.fundamental_boost > 0.001 {
        for i in 0..WT_SIZE {
            let fund = (i as f32 / WT_SIZE as f32 * TAU).sin();
            baked[i] += fund * state.fundamental_boost;
        }
    }

    // 3. Wavefold
    if state.wavefold_amount > 0.001 {
        let fold_gain = 1.0 + state.wavefold_amount * 4.0;
        for s in baked.iter_mut() {
            *s = (*s * fold_gain).sin(); // simple sine wavefold
        }
    }
    
    // Normalize if needed
    let mut max_val: f32 = 0.0;
    for &s in &baked { if s.abs() > max_val { max_val = s.abs(); } }
    if max_val > 0.0001 && max_val > 1.0 {
        for s in baked.iter_mut() { *s /= max_val; }
    }
    
    state.baked_table = baked;
}
