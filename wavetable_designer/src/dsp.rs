use crate::app_state::{WavetableFrame, WtState, WT_SIZE};
use crate::app_state::{WtState, WT_SIZE};
use num_complex::Complex;
use rustfft::FftPlanner;
use std::f32::consts::TAU;

/// Runs the full bake pipeline on the active frame and writes the result to
/// `frame.baked`.
///
/// The pipeline applies, in order:
/// 1. 2-op FM stacking (modulates the carrier phase using a separate oscillator).
/// 2. Fundamental boost (adds a sine wave at the fundamental frequency).
/// 3. Wavefolding (sine-based soft fold).
/// 4. Peak normalisation (if the output exceeds ±1).
pub fn bake_wavetable(state: &mut WtState) {
    let raw = state.active_frame().raw.clone();
    let baked = bake_frame(&raw, state);
    state.active_frame_mut().baked = baked;
}

fn bake_frame(raw: &[f32], state: &WtState) -> Vec<f32> {
    let mut baked = raw.to_vec();

    // 1. FM Stacking (2-op)
    if state.fm_amount > 0.001 {
        baked = apply_fm_stack(raw, state.fm_ratio, state.fm_amount, state.mod_shape);
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
            *s = (*s * fold_gain).sin();
        }
    }

    normalize_samples_in_place(&mut baked);

    // 4. Spectral FX
    if state.spectral_formant != 0.0 || state.spectral_smear > 0.001 || state.spectral_stretch > 0.001 || state.spectral_warp != 0.0 {
        apply_spectral_fx(&mut baked, state);
        normalize_samples_in_place(&mut baked);
    }

    baked
}

/// Computes the FFT magnitude spectrum of `samples` and returns the first half
/// (positive frequencies), scaled by `1 / N`.
///
/// Returns an empty `Vec` if `samples` is empty.
fn apply_spectral_fx(_samples: &mut [f32], _state: &WtState) {
    // Placeholder for upcoming spectral processing (formant, smear, stretch, warp).
    // Kept as a no-op for now to preserve the control surface without changing audio.
}

pub fn apply_fm_stack(raw: &[f32], ratio: f32, amount: f32, mod_shape: usize) -> Vec<f32> {
    let len = raw.len();
    if len == 0 {
        return Vec::new();
    }
    if amount <= 0.001 {
        return raw.to_vec();
    }

    let mut fm_table = vec![0.0; len];
    for i in 0..len {
        // Modulator phase
        let mod_phase = (i as f32 / len as f32 * ratio * TAU) % TAU;
        let mod_out = match mod_shape {
            0 => mod_phase.sin(),
            1 => std::f32::consts::FRAC_1_PI * (mod_phase - std::f32::consts::PI), // saw
            2 => {
                if mod_phase < std::f32::consts::PI {
                    1.0
                } else {
                    -1.0
                }
            }
            _ => 0.0,
        };

        // Carrier phase (modulated)
        let base_phase = i as f32 / len as f32;
        let carrier_phase = (base_phase + (mod_out * amount / TAU)).fract();
        let mut c_idx = (carrier_phase * len as f32) as usize;
        if c_idx >= len {
            c_idx = len - 1;
        }
        fm_table[i] = raw[c_idx];
    }
    fm_table
}

pub fn compute_harmonics(samples: &[f32]) -> Vec<f32> {
    let len = samples.len();
    if len == 0 {
        return Vec::new();
    }

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(len);
    let mut buffer: Vec<Complex<f32>> = samples
        .iter()
        .map(|&s| Complex { re: s, im: 0.0 })
        .collect();
    fft.process(&mut buffer);

    let half = len / 2;
    let scale = len as f32;
    buffer
        .iter()
        .take(half)
        .map(|c| c.norm() / scale)
        .collect()
}

/// Converts a MIDI note number plus a fine-tune offset in cents to a frequency
/// in Hz using the standard equal-temperament formula.
///
/// `detune_cents` of ±1200 is equivalent to ±one octave.
pub fn note_to_freq(note: u8, detune_cents: f32) -> f32 {
    let base = nih_plug::util::midi_note_to_freq(note);
    let ratio = 2.0_f32.powf(detune_cents / 1200.0);
    base * ratio
}

/// Computes the forward (analysis) FFT of `samples` and returns the complex
/// frequency-domain bins.
///
/// Returns an empty `Vec` if `samples` is empty.
#[allow(dead_code)]
pub fn forward_fft(samples: &[f32]) -> Vec<Complex<f32>> {
    let len = samples.len();
    if len == 0 {
        return Vec::new();
    }
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(len);
    let mut buffer: Vec<Complex<f32>> = samples
        .iter()
        .map(|&s| Complex { re: s, im: 0.0 })
        .collect();
    fft.process(&mut buffer);
    buffer
}

/// Computes the inverse FFT of `bins` and returns the real-valued time-domain
/// samples, scaled by `1 / N` to invert the forward transform.
///
/// Returns an empty `Vec` if `bins` is empty.
#[allow(dead_code)]
pub fn inverse_fft(bins: &[Complex<f32>]) -> Vec<f32> {
    let len = bins.len();
    if len == 0 {
        return Vec::new();
    }
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_inverse(len);
    let mut buffer = bins.to_vec();
    fft.process(&mut buffer);

    let scale = 1.0 / len as f32;
    buffer.iter().map(|c| c.re * scale).collect()
}

/// Enforces Hermitian (conjugate) symmetry on a complex FFT bin array so that
/// its inverse transform yields a purely real-valued signal.
///
/// Sets `bins[N-i] = conj(bins[i])` for `i` in `1..N/2`, and zeroes the
/// imaginary parts of DC (bin 0) and the Nyquist bin (bin N/2 when N is even).
#[allow(dead_code)]
pub fn enforce_conjugate_symmetry(bins: &mut [Complex<f32>]) {
    let len = bins.len();
    if len <= 1 {
        return;
    }
    for i in 1..(len / 2) {
        bins[len - i] = bins[i].conj();
    }
    bins[0].im = 0.0;
    if len % 2 == 0 {
        bins[len / 2].im = 0.0;
    }
}

// FFT Context Menu Operations
pub fn fft_clear_all(bins: &mut [Complex<f32>]) {
    for b in bins.iter_mut() {
        *b = Complex::new(0.0, 0.0);
    }
}

pub fn fft_clear_hf(bins: &mut [Complex<f32>], start_bin: usize) {
    let len = bins.len();
    if start_bin >= len / 2 { return; }
    for i in start_bin..(len / 2) {
        bins[i] = Complex::new(0.0, 0.0);
    }
    enforce_conjugate_symmetry(bins);
}

pub fn fft_clear_lf(bins: &mut [Complex<f32>], end_bin: usize) {
    let len = bins.len();
    let end = end_bin.min(len / 2);
    for i in 1..=end {
        bins[i] = Complex::new(0.0, 0.0);
    }
    enforce_conjugate_symmetry(bins);
}

pub fn fft_generate_saw(bins: &mut [Complex<f32>]) {
    let len = bins.len();
    for i in 1..(len / 2) {
        let mag = 1.0 / i as f32; // Sawtooth amplitude drops by 1/n
        // Real part 0, imaginary part dictates sine out of phase (or just negative imag for standard phasing)
        bins[i] = Complex::new(0.0, -mag); 
    }
    bins[0] = Complex::new(0.0, 0.0);
    enforce_conjugate_symmetry(bins);
}

pub fn fft_randomize_bins(bins: &mut [Complex<f32>], num_bins: usize) {
    let len = bins.len();
    let end = num_bins.min(len / 2);
    for i in 1..end {
        let mag = rand::random::<f32>();
        let phase = rand::random::<f32>() * std::f32::consts::TAU;
        bins[i] = Complex::new(mag * phase.cos(), mag * phase.sin());
    }
    enforce_conjugate_symmetry(bins);
}

pub fn fft_draw_even_only(bins: &mut [Complex<f32>]) {
    let len = bins.len();
    for i in 1..(len / 2) {
        if i % 2 != 0 {
            bins[i] = Complex::new(0.0, 0.0);
        }
    }
    enforce_conjugate_symmetry(bins);
}

pub fn fft_draw_odd_only(bins: &mut [Complex<f32>]) {
    let len = bins.len();
    for i in 1..(len / 2) {
        if i % 2 == 0 {
            bins[i] = Complex::new(0.0, 0.0);
        }
    }
    enforce_conjugate_symmetry(bins);
}

pub fn spectral_morph_preview(a: &[f32], b: &[f32], amount: f32) -> Vec<f32> {
    let len = a.len().min(b.len());
    if len == 0 {
        return Vec::new();
    }

    let amount = amount.clamp(0.0, 1.0);
    let a_bins = forward_fft(&a[..len]);
    let b_bins = forward_fft(&b[..len]);

    let mut out_bins = Vec::with_capacity(len);
    for i in 0..len {
        let mag_a = a_bins[i].norm();
        let mag_b = b_bins[i].norm();
        let phase = a_bins[i].arg();
        let mag = mag_a + (mag_b - mag_a) * amount;
        out_bins.push(Complex::from_polar(mag, phase));
    }
    enforce_conjugate_symmetry(&mut out_bins);
    let mut out = inverse_fft(&out_bins);
    normalize_samples_in_place(&mut out);
    out
}

pub fn apply_spectral_fx(samples: &mut [f32], state: &WtState) {
    let mut bins = forward_fft(samples);
    let len = bins.len();
    if len <= 1 { return; }
    
    let half = len / 2;
    let mut polar: Vec<(f32, f32)> = bins.iter().map(|c| (c.norm(), c.arg())).collect();
    
    if state.spectral_smear > 0.001 {
        let mut blurred = polar.clone();
        let smear_bins = (state.spectral_smear * 15.0) as usize;
        for i in 1..half {
            let mut sum_mag = 0.0;
            let mut count = 0;
            let start = i.saturating_sub(smear_bins).max(1);
            let end = (i + smear_bins).min(half - 1);
            for j in start..=end {
                sum_mag += polar[j].0;
                count += 1;
            }
            if count > 0 {
                blurred[i].0 = sum_mag / count as f32;
            }
        }
        polar = blurred;
    }

    if state.spectral_warp != 0.0 || state.spectral_stretch > 0.001 {
        let mut warped = vec![(0.0, 0.0); len];
        for i in 1..half {
            let norm_x = i as f32 / half as f32;
            let stretch_amt = state.spectral_stretch * 0.9; 
            let warp_factor = if state.spectral_warp >= 0.0 {
                1.0 + state.spectral_warp * 3.0
            } else {
                1.0 / (1.0 - state.spectral_warp * 3.0)
            };
            let mapped_x = norm_x.powf(warp_factor) * (1.0 - stretch_amt).max(0.1);
            let mapped_i = (mapped_x * half as f32) as usize;
            if mapped_i > 0 && mapped_i < half {
                warped[mapped_i].0 += polar[i].0;
                warped[mapped_i].1 = polar[i].1; 
            }
        }
        polar = warped;
    }

    if state.spectral_formant != 0.0 {
        let shift = (state.spectral_formant * 30.0) as isize;
        let mut shifted = vec![(0.0, 0.0); len];
        for i in 1..half {
            let target = i as isize + shift;
            if target > 0 && target < half as isize {
                shifted[target as usize] = polar[i];
            }
        }
        polar = shifted;
    }

    for i in 1..half {
        bins[i] = Complex::from_polar(polar[i].0, polar[i].1);
    }
    
    enforce_conjugate_symmetry(&mut bins);
    let out = inverse_fft(&bins);
    samples.copy_from_slice(&out);
}


fn normalize_samples_in_place(samples: &mut [f32]) {
    let mut max_val: f32 = 0.0;
    for &s in samples.iter() {
        let abs = s.abs();
        if abs > max_val {
            max_val = abs;
        }
    }
    if max_val > 1.0 {
        let inv = 1.0 / max_val;
        for s in samples.iter_mut() {
            *s *= inv;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bake_wavetable_is_finite_and_normalized() {
        let mut state = WtState::default();
        state.fm_amount = 2.0;
        state.wavefold_amount = 0.8;
        bake_wavetable(&mut state);

        let frame = state.active_frame();
        let mut max_val = 0.0f32;
        for &sample in &frame.baked {
            assert!(sample.is_finite());
            if sample.abs() > max_val {
                max_val = sample.abs();
            }
        }
        assert!(max_val <= 1.0 + 1e-6);
    }

    #[test]
    fn note_to_freq_detune() {
        let base = note_to_freq(69, 0.0); // A4
        let octave_up = note_to_freq(69, 1200.0);
        assert!((base - 440.0).abs() < 0.5);
        assert!((octave_up - 880.0).abs() < 1.0);
    }

    #[test]
    fn fft_roundtrip() {
        let mut original = vec![0.0f32; 256];
        for i in 0..256 {
            original[i] = (i as f32 / 256.0 * std::f32::consts::TAU * 4.0).sin();
        }

        let bins = forward_fft(&original);
        let reconstructed = inverse_fft(&bins);

        for (o, r) in original.iter().zip(reconstructed.iter()) {
            assert!((o - r).abs() < 1e-4);
        }
    }
}
