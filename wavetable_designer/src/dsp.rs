use crate::app_state::{WavetableFrame, WtState, WT_SIZE};
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
            if c_idx >= WT_SIZE {
                c_idx = WT_SIZE - 1;
            }
            fm_table[i] = raw[c_idx];
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
            *s = (*s * fold_gain).sin();
        }
    }

    // Normalize if needed
    let mut max_val: f32 = 0.0;
    for &s in &baked {
        if s.abs() > max_val {
            max_val = s.abs();
        }
    }
    if max_val > 0.0001 && max_val > 1.0 {
        for s in baked.iter_mut() {
            *s /= max_val;
        }
    }

    baked
}

/// Computes the FFT magnitude spectrum of `samples` and returns the first half
/// (positive frequencies), scaled by `1 / N`.
///
/// Returns an empty `Vec` if `samples` is empty.
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
