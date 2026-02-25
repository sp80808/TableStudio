use nih_plug_egui::egui::{self, Color32, Stroke, Vec2, Rect};
use crate::app_state::{WtState, WT_SIZE};
use parking_lot::Mutex;
use std::sync::Arc;
use crate::dsp::bake_wavetable;

pub fn draw_ui(
    ctx: &egui::Context,
    setter: &nih_plug::prelude::ParamSetter,
    state: &Arc<Mutex<WtState>>,
    params: &Arc<crate::WtParams>,
) {
    let mut state_guard = state.lock();
    let mut needs_bake = false;

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("TableStudio Wavetable Designer 🔥");

        ui.horizontal(|ui| {
            if ui.button("Sine").clicked() {
                for (i, s) in state_guard.raw_table.iter_mut().enumerate() {
                    *s = (i as f32 / WT_SIZE as f32 * std::f32::consts::TAU).sin();
                }
                needs_bake = true;
            }
            if ui.button("Saw").clicked() {
                for (i, s) in state_guard.raw_table.iter_mut().enumerate() {
                    *s = (i as f32 / WT_SIZE as f32) * -2.0 + 1.0;
                }
                needs_bake = true;
            }
            if ui.button("Square").clicked() {
                for (i, s) in state_guard.raw_table.iter_mut().enumerate() {
                    *s = if i < WT_SIZE / 2 { 1.0 } else { -1.0 };
                }
                needs_bake = true;
            }
            if ui.button("Export .wav").clicked() {
                let path = std::env::temp_dir().join("tablestudio_wt.wav");
                let spec = hound::WavSpec { channels: 1, sample_rate: 44100, bits_per_sample: 32, sample_format: hound::SampleFormat::Float };
                if let Ok(mut writer) = hound::WavWriter::create(&path, spec) {
                    for &sample in &state_guard.baked_table {
                        let _ = writer.write_sample(sample);
                    }
                    if writer.finalize().is_ok() {
                        println!("✅ Exported to: {}", path.display());
                    }
                }
            }
        });

        ui.separator();

        // Dual Wavetable Display (Raw Blue vs Baked Orange)
        let canvas_size = Vec2::new(ui.available_width(), 200.0);
        let (rect, response) = ui.allocate_exact_size(canvas_size, egui::Sense::drag());

        let painter = ui.painter_at(rect);
        painter.rect_filled(rect, 4.0, Color32::from_gray(30));

        // Grid
        for i in 1..8 {
            let x = rect.min.x + (i as f32 / 8.0) * rect.width();
            painter.vline(x, rect.y_range(), Stroke::new(1.0, Color32::from_gray(50)));
        }
        painter.hline(rect.x_range(), rect.center().y, Stroke::new(1.0, Color32::from_gray(50)));

        // Draw Baked (Orange Glow)
        let mut baked_points = Vec::with_capacity(WT_SIZE);
        for (i, &sample) in state_guard.baked_table.iter().enumerate() {
            let x = rect.min.x + (i as f32 / WT_SIZE as f32) * rect.width();
            let y = rect.center().y - sample * (rect.height() * 0.45);
            baked_points.push(egui::pos2(x, y));
        }
        painter.add(egui::Shape::line(baked_points, Stroke::new(3.0, Color32::from_rgb(255, 140, 0))));

        // Draw Raw (Blue Edition)
        let mut raw_points = Vec::with_capacity(WT_SIZE);
        for (i, &sample) in state_guard.raw_table.iter().enumerate() {
            let x = rect.min.x + (i as f32 / WT_SIZE as f32) * rect.width();
            let y = rect.center().y - sample * (rect.height() * 0.45);
            raw_points.push(egui::pos2(x, y));
        }
        painter.add(egui::Shape::line(raw_points, Stroke::new(1.5, Color32::from_rgb(50, 150, 255))));

        // Drag to Draw
        if response.dragged() || response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let x_norm = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
                let idx = (x_norm * (WT_SIZE - 1) as f32) as usize;
                let y_norm = ((rect.center().y - pos.y) / (rect.height() * 0.45)).clamp(-1.0, 1.0);
                state_guard.raw_table[idx] = y_norm;
                
                // Neighbor smoothing
                if idx > 0 { state_guard.raw_table[idx - 1] = state_guard.raw_table[idx - 1] * 0.7 + y_norm * 0.3; }
                if idx < WT_SIZE - 1 { state_guard.raw_table[idx + 1] = state_guard.raw_table[idx + 1] * 0.7 + y_norm * 0.3; }
                
                needs_bake = true;
            }
        }

        ui.separator();

        ui.columns(2, |cols| {
            // == Column 1: FM Injector ==
            cols[0].group(|ui| {
                ui.heading("FM Stacking Pipeline");
                
                if ui.add(egui::Slider::new(&mut state_guard.fm_amount, 0.0..=10.0).text("FM Amount")).changed() { needs_bake = true; }
                if ui.add(egui::Slider::new(&mut state_guard.fm_ratio, 0.1..=16.0).text("FM Ratio")).changed() { needs_bake = true; }
                
                ui.horizontal(|ui| {
                    ui.label("Mod Shape:");
                    if ui.radio_value(&mut state_guard.mod_shape, 0, "Sine").changed() { needs_bake = true; }
                    if ui.radio_value(&mut state_guard.mod_shape, 1, "Saw").changed() { needs_bake = true; }
                    if ui.radio_value(&mut state_guard.mod_shape, 2, "Square").changed() { needs_bake = true; }
                });

                if ui.button("Bake FM into Core (Destructive)").clicked() {
                    state_guard.raw_table = state_guard.baked_table.clone();
                    state_guard.fm_amount = 0.0;
                    needs_bake = true;
                }
            });

            // == Column 2: BassForge Panel ==
            cols[1].group(|ui| {
                ui.heading("BassForge Panel");
                ui.label("The Bass Doctor Analyzer: Checking Sub Power...");
                
                if ui.add(egui::Slider::new(&mut state_guard.fundamental_boost, 0.0..=2.0).text("Fundamental Boost")).changed() { needs_bake = true; }
                if ui.add(egui::Slider::new(&mut state_guard.wavefold_amount, 0.0..=1.0).text("Wavefolder Amount")).changed() { needs_bake = true; }
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Preview Frequency (Hz):");
            ui.add(egui::DragValue::new(&mut state_guard.preview_freq).range(20.0..=2000.0).speed(1.0));
            
            ui.toggle_value(&mut state_guard.preview_playing, "🔊 PLAY LIVE AUDIO");
            
            ui.add(nih_plug_egui::widgets::ParamSlider::for_param(&params.preview_gain, setter));
        });

        // Drop files
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files = ctx.input(|i| i.raw.dropped_files.clone());
            if let Some(file) = files.first() {
                if let Some(path) = &file.path {
                    if let Ok(mut reader) = hound::WavReader::open(path) {
                        let samples: Vec<f32> = reader.samples::<f32>().filter_map(Result::ok).collect();
                        if samples.len() >= WT_SIZE {
                            state_guard.raw_table.copy_from_slice(&samples[..WT_SIZE]);
                            needs_bake = true;
                        }
                    }
                }
            }
        }
    });

    if needs_bake {
        bake_wavetable(&mut state_guard);
    }
}
