use crate::app_state::{MorphMode, WtState, WT_SIZE};
use crate::dsp::{bake_wavetable, execute_morph};
use crate::WtParams;
use hound::SampleFormat;
use nih_plug_egui::egui::{self};
use parking_lot::Mutex;
use std::path::Path;
use std::sync::Arc;

mod canvas;
mod grid;
mod preview;

pub fn draw_ui(
    ctx: &egui::Context,
    setter: &nih_plug::prelude::ParamSetter,
    state: &Arc<Mutex<WtState>>,
    params: &Arc<WtParams>,
) {
    let mut state_guard = state.lock();
    let mut needs_bake = false;
    let mut status_msg: Option<String> = None;

    egui::SidePanel::left("frame_grid")
        .resizable(false)
        .default_width(220.0)
        .show(ctx, |ui| {
            ui.heading("Frames");
            ui.separator();
            if grid::draw_frame_grid(ui, &mut state_guard) {
                needs_bake = true;
            }
        });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("TableStudio Wavetable Designer");
        ui.horizontal(|ui| {
            let total_frames = state_guard.frames.len();
            let active_index = state_guard.active_frame + 1;
            ui.label(format!("Active Frame: {}/{}", active_index, total_frames));
            let frame = state_guard.active_frame_mut();
            if ui
                .toggle_value(&mut frame.is_keyframe, "Keyframe")
                .changed()
            {
                // no-op for now, keyframes used for future morphing
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Import WAV").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("WAV audio", &["wav"])
                    .pick_file()
                {
                    match import_wavetable(&path, &mut state_guard) {
                        Ok(msg) => {
                            status_msg = Some(msg);
                            needs_bake = true;
                        }
                        Err(err) => status_msg = Some(err),
                    }
                }
            }

            if ui.button("Export Current").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_file_name("wavetable_frame.wav")
                    .save_file()
                {
                    match export_current_frame(&path, &state_guard) {
                        Ok(msg) => status_msg = Some(msg),
                        Err(err) => status_msg = Some(err),
                    }
                }
            }

            if ui.button("Export All").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .set_file_name("wavetable_all.wav")
                    .save_file()
                {
                    match export_all_frames(&path, &state_guard) {
                        Ok(msg) => status_msg = Some(msg),
                        Err(err) => status_msg = Some(err),
                    }
                }
            }

            ui.separator();
            ui.menu_button("MORPH", |ui| {
                if ui.button("Morph - Crossfade").clicked() {
                    execute_morph(&mut state_guard.frames, MorphMode::Crossfade);
                    needs_bake = true;
                    ui.close_menu();
                }
                if ui.button("Morph - Spectral").clicked() {
                    execute_morph(&mut state_guard.frames, MorphMode::Spectral);
                    needs_bake = true;
                    ui.close_menu();
                }
                if ui.button("Morph - Spectral (Zero All Phases)").clicked() {
                    execute_morph(&mut state_guard.frames, MorphMode::SpectralZeroPhase);
                    needs_bake = true;
                    ui.close_menu();
                }
            });
        });

        ui.separator();

        needs_bake |= canvas::draw_canvas(ui, ctx, &mut state_guard);

        ui.separator();

        ui.columns(2, |cols| {
            cols[0].group(|ui| {
                ui.heading("FM Stacking Pipeline");
                ui.horizontal(|ui| {
                    if crate::widgets::synth_knob(ui, &mut state_guard.fm_amount, 0.0..=10.0, "Amount").changed() { needs_bake = true; }
                    if crate::widgets::synth_knob(ui, &mut state_guard.fm_ratio, 0.1..=16.0, "Ratio").changed() { needs_bake = true; }
                });

                ui.horizontal(|ui| {
                    ui.label("Mod Shape:");
                    if ui
                        .radio_value(&mut state_guard.mod_shape, 0, "Sine")
                        .changed()
                    {
                        needs_bake = true;
                    }
                    if ui
                        .radio_value(&mut state_guard.mod_shape, 1, "Saw")
                        .changed()
                    {
                        needs_bake = true;
                    }
                    if ui
                        .radio_value(&mut state_guard.mod_shape, 2, "Square")
                        .changed()
                    {
                        needs_bake = true;
                    }
                });

                if ui.button("Bake FM into Core (Destructive)").clicked() {
                    let frame = state_guard.active_frame_mut();
                    frame.raw = frame.baked.clone();
                    state_guard.fm_amount = 0.0;
                    needs_bake = true;
                }
            });

            cols[1].group(|ui| {
                ui.heading("BassForge Panel");
                ui.label("The Bass Doctor Analyzer: Checking Sub Power...");

                ui.horizontal(|ui| {
                    if crate::widgets::synth_knob(ui, &mut state_guard.fundamental_boost, 0.0..=2.0, "Fund. Boost").changed() { needs_bake = true; }
                    if crate::widgets::synth_knob(ui, &mut state_guard.wavefold_amount, 0.0..=1.0, "Wavefold").changed() { needs_bake = true; }
                    if crate::widgets::synth_knob(ui, &mut state_guard.spectral_smear, 0.0..=1.0, "Smear").changed() { needs_bake = true; }
                    if crate::widgets::synth_knob(ui, &mut state_guard.spectral_warp, -1.0..=1.0, "Warp").changed() { needs_bake = true; }
                });
                ui.horizontal(|ui| {
                    if crate::widgets::synth_knob(ui, &mut state_guard.spectral_stretch, 0.0..=1.0, "Stretch").changed() { needs_bake = true; }
                    if crate::widgets::synth_knob(ui, &mut state_guard.spectral_formant, -1.0..=1.0, "Formant").changed() { needs_bake = true; }
                });
            });
        });

        ui.separator();

        ui.group(|ui| {
            ui.heading("Spectral Morph (WIP)");
            ui.label("Controls reserved for upcoming spectral processing.");
            ui.horizontal_wrapped(|ui| {
                if crate::widgets::synth_knob(
                    ui,
                    &mut state_guard.spectral_morph_amount,
                    0.0..=1.0,
                    "Morph",
                )
                .changed()
                {
                    needs_bake = true;
                }
                if crate::widgets::synth_knob(
                    ui,
                    &mut state_guard.spectral_formant,
                    -1.0..=1.0,
                    "Formant",
                )
                .changed()
                {
                    needs_bake = true;
                }
                if crate::widgets::synth_knob(
                    ui,
                    &mut state_guard.spectral_smear,
                    0.0..=1.0,
                    "Smear",
                )
                .changed()
                {
                    needs_bake = true;
                }
                if crate::widgets::synth_knob(
                    ui,
                    &mut state_guard.spectral_stretch,
                    0.0..=2.0,
                    "Stretch",
                )
                .changed()
                {
                    needs_bake = true;
                }
                if crate::widgets::synth_knob(
                    ui,
                    &mut state_guard.spectral_warp,
                    -1.0..=1.0,
                    "Warp",
                )
                .changed()
                {
                    needs_bake = true;
                }
            });
        });

        ui.separator();

        preview::draw_preview_panel(ui, &mut state_guard, params, setter);

        if let Some(ref msg) = status_msg {
            ui.separator();
            ui.label(msg);
        }

        if handle_file_drop(ctx, &mut state_guard, &mut status_msg) {
            needs_bake = true;
        }
    });

    if needs_bake {
        bake_wavetable(&mut state_guard);
    }
}

fn handle_file_drop(
    ctx: &egui::Context,
    state: &mut WtState,
    status_msg: &mut Option<String>,
) -> bool {
    let mut changed = false;
    if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
        let files = ctx.input(|i| i.raw.dropped_files.clone());
        if let Some(file) = files.first() {
            if let Some(path) = &file.path {
                match import_wavetable(path, state) {
                    Ok(msg) => {
                        *status_msg = Some(msg);
                        changed = true;
                    }
                    Err(err) => *status_msg = Some(err),
                }
            }
        }
    }

    changed
}

fn import_wavetable(path: &Path, state: &mut WtState) -> Result<String, String> {
    let samples = load_wav_mono(path)?;

    if samples.len() >= WT_SIZE {
        let frame_count = samples.len() / WT_SIZE;
        let frames_to_load = frame_count.min(state.frames.len());
        for i in 0..frames_to_load {
            let start = i * WT_SIZE;
            let end = start + WT_SIZE;
            let frame = &mut state.frames[i];
            frame.raw.copy_from_slice(&samples[start..end]);
            frame.baked.copy_from_slice(&samples[start..end]);
        }
        state.active_frame = 0;
        Ok(format!(
            "Imported {} frames from {}",
            frames_to_load,
            path.display()
        ))
    } else {
        let resampled = resample_to_len(&samples, WT_SIZE);
        let frame = state.active_frame_mut();
        frame.raw.copy_from_slice(&resampled);
        frame.baked.copy_from_slice(&resampled);
        Ok(format!("Imported frame from {}", path.display()))
    }
}

fn export_current_frame(path: &Path, state: &WtState) -> Result<String, String> {
    let frame = state.active_frame();
    write_wav(path, &frame.baked)?;
    Ok(format!("Exported current frame to {}", path.display()))
}

fn export_all_frames(path: &Path, state: &WtState) -> Result<String, String> {
    let mut data = Vec::with_capacity(state.frames.len() * WT_SIZE);
    for frame in &state.frames {
        data.extend_from_slice(&frame.baked);
    }
    write_wav(path, &data)?;
    Ok(format!("Exported all frames to {}", path.display()))
}

fn load_wav_mono(path: &Path) -> Result<Vec<f32>, String> {
    let mut reader = hound::WavReader::open(path)
        .map_err(|err| format!("Failed to open WAV: {err}"))?;
    let spec = reader.spec();
    let channels = spec.channels.max(1) as usize;

    let raw: Vec<f32> = match spec.sample_format {
        SampleFormat::Float => reader
            .samples::<f32>()
            .map(|s| s.map_err(|e| e.to_string()))
            .collect::<Result<_, _>>()?,
        SampleFormat::Int => {
            let max_amplitude = (1u64 << (spec.bits_per_sample.saturating_sub(1))) as f32;
            reader
                .samples::<i32>()
                .map(|s| s.map(|v| v as f32 / max_amplitude).map_err(|e| e.to_string()))
                .collect::<Result<_, _>>()?
        }
    };

    if raw.is_empty() {
        return Err("WAV contains no samples".to_string());
    }

    let mut mono = Vec::with_capacity(raw.len() / channels);
    for chunk in raw.chunks(channels) {
        mono.push(chunk[0]);
    }

    Ok(mono)
}

fn resample_to_len(samples: &[f32], target_len: usize) -> Vec<f32> {
    if samples.is_empty() {
        return vec![0.0; target_len];
    }

    let src_len = samples.len() as f32;
    let mut out = vec![0.0; target_len];
    let last = target_len.saturating_sub(1) as f32;

    for i in 0..target_len {
        let pos = if last > 0.0 {
            (i as f32 / last) * (src_len - 1.0)
        } else {
            0.0
        };
        let idx = pos.floor() as usize;
        let frac = pos - idx as f32;
        let a = samples[idx];
        let b = samples.get(idx + 1).copied().unwrap_or(a);
        out[i] = a + (b - a) * frac;
    }

    out
}

fn write_wav(path: &Path, data: &[f32]) -> Result<(), String> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44_100,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(path, spec)
        .map_err(|err| format!("Failed to create WAV: {err}"))?;
    for &sample in data {
        writer
            .write_sample(sample)
            .map_err(|err| format!("Failed to write WAV: {err}"))?;
    }
    writer
        .finalize()
        .map_err(|err| format!("Failed to finalize WAV: {err}"))?;
    Ok(())
}
