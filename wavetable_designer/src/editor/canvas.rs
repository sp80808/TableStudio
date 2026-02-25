use crate::app_state::{WtState, WT_SIZE};
use crate::dsp::{
    apply_fm_stack, compute_harmonics, enforce_conjugate_symmetry, fft_clear_all, fft_clear_hf,
    fft_clear_lf, fft_draw_even_only, fft_draw_odd_only, fft_generate_saw, fft_randomize_bins,
    forward_fft, inverse_fft, spectral_morph_preview,
};
use nih_plug_egui::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};

const Y_SCALE: f32 = 0.45;
const GRID_DIVS: usize = 8;
const HARMONICS_BINS: usize = 128;

pub fn draw_canvas(ui: &mut egui::Ui, _ctx: &egui::Context, state: &mut WtState) -> bool {
    let mut needs_bake = false;

    let canvas_size = Vec2::new(ui.available_width(), 220.0);
    let (rect, response) = ui.allocate_exact_size(canvas_size, Sense::drag());

    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4.0, Color32::from_gray(30));

    for i in 1..GRID_DIVS {
        let x = rect.min.x + (i as f32 / GRID_DIVS as f32) * rect.width();
        painter.vline(x, rect.y_range(), Stroke::new(1.0, Color32::from_gray(50)));
    }
    painter.hline(
        rect.x_range(),
        rect.center().y,
        Stroke::new(1.0, Color32::from_gray(60)),
    );

    {
        let frame = state.active_frame();

        let fm_preview = apply_fm_stack(&frame.raw, state.fm_ratio, state.fm_amount, state.mod_shape);
        let spectral_preview = spectral_morph_preview(
            &frame.raw,
            &fm_preview,
            state.spectral_morph_amount,
        );

        let mut raw_points = Vec::with_capacity(WT_SIZE);
        for (i, &sample) in frame.raw.iter().enumerate() {
            raw_points.push(sample_to_pos(rect, i, sample));
        }
        painter.add(egui::Shape::line(
            raw_points,
            Stroke::new(1.4, Color32::from_rgb(50, 150, 255)),
        ));

        let mut fm_points = Vec::with_capacity(WT_SIZE);
        for (i, &sample) in fm_preview.iter().enumerate() {
            fm_points.push(sample_to_pos(rect, i, sample));
        }
        painter.add(egui::Shape::line(
            fm_points,
            Stroke::new(2.4, Color32::from_rgb(255, 140, 0)),
        ));

        let mut spectral_points = Vec::with_capacity(WT_SIZE);
        for (i, &sample) in spectral_preview.iter().enumerate() {
            spectral_points.push(sample_to_pos(rect, i, sample));
        }
        painter.add(egui::Shape::line(
            spectral_points,
            Stroke::new(2.0, Color32::from_rgb(170, 80, 255)),
        ));
    }

    if response.dragged() || response.clicked() {
        if let Some(pos) = response.interact_pointer_pos() {
            let (idx, value) = sample_from_pos(rect, pos);
            let frame = state.active_frame_mut();
            frame.raw[idx] = value;

            if idx > 0 {
                frame.raw[idx - 1] = frame.raw[idx - 1] * 0.7 + value * 0.3;
            }
            if idx + 1 < WT_SIZE {
                frame.raw[idx + 1] = frame.raw[idx + 1] * 0.7 + value * 0.3;
            }

            needs_bake = true;
        }
    }

    state.edit_gate = response.dragged();

    ui.add_space(8.0);
    needs_bake |= draw_harmonics(ui, state);

    if needs_bake {
        state.active_frame_mut().is_keyframe = true;
    }

    needs_bake
}

fn draw_harmonics(ui: &mut egui::Ui, state: &mut WtState) -> bool {
    let mut needs_bake = false;
    let frame = state.active_frame();

    ui.label("Harmonics");
    let harmonics = compute_harmonics(&frame.baked);
    let bins = harmonics.len().min(HARMONICS_BINS);

    let (rect, response) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 120.0), Sense::click());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4.0, Color32::from_gray(24));

    let max_val = harmonics
        .iter()
        .take(bins)
        .fold(0.0_f32, |acc, &v| acc.max(v));
    let max_val = if max_val > 0.0 { max_val } else { 1.0 };

    if bins > 1 {
        for i in 0..bins {
            let x = rect.left() + (i as f32 / (bins - 1) as f32) * rect.width();
            let magnitude = harmonics[i] / max_val;
            let y = rect.bottom() - magnitude * rect.height();
            painter.vline(
                x,
                y..=rect.bottom(),
                Stroke::new(1.0, Color32::from_rgb(200, 200, 120)),
            );
        }
    }

    // Context menu for FFT Operations
    response.context_menu(|ui| {
        ui.menu_button("FFT Operations", |ui| {
            let process_fft = |ui: &mut egui::Ui, name: &str, mut op: impl FnMut(&mut [num_complex::Complex<f32>])| -> bool {
                if ui.button(name).clicked() {
                    ui.close_menu();
                    return true;
                }
                false
            };

            let mut run_op = None;

            if ui.button("Clear All").clicked() { run_op = Some(0); }
            if ui.button("Clear HF (Bin 11 to End)").clicked() { run_op = Some(1); }
            if ui.button("Clear LF (Start to 11)").clicked() { run_op = Some(2); }
            ui.separator();
            if ui.button("Generate Saw").clicked() { run_op = Some(3); }
            if ui.button("Randomize Low 16 Bins").clicked() { run_op = Some(4); }
            if ui.button("Randomize Low 32 Bins").clicked() { run_op = Some(5); }
            if ui.button("Randomize All").clicked() { run_op = Some(6); }
            ui.separator();
            if ui.button("Draw Even Harmonics Only").clicked() { run_op = Some(7); }
            if ui.button("Draw Odd Harmonics Only").clicked() { run_op = Some(8); }

            if let Some(op_idx) = run_op {
                let frame = state.active_frame_mut();
                let mut fft_bins = forward_fft(&frame.raw);

                match op_idx {
                    0 => fft_clear_all(&mut fft_bins),
                    1 => fft_clear_hf(&mut fft_bins, 11),
                    2 => fft_clear_lf(&mut fft_bins, 11),
                    3 => fft_generate_saw(&mut fft_bins),
                    4 => fft_randomize_bins(&mut fft_bins, 16),
                    5 => fft_randomize_bins(&mut fft_bins, 32),
                    6 => {
                        let len = fft_bins.len();
                        fft_randomize_bins(&mut fft_bins, len / 2);
                    },
                    7 => fft_draw_even_only(&mut fft_bins),
                    8 => fft_draw_odd_only(&mut fft_bins),
                    _ => {}
                }

                frame.raw = inverse_fft(&fft_bins);
                needs_bake = true;
                ui.close_menu();
            }
        });
    });

    needs_bake
}

fn sample_to_pos(rect: Rect, index: usize, sample: f32) -> Pos2 {
    let x = rect.min.x + (index as f32 / WT_SIZE as f32) * rect.width();
    let y = rect.center().y - sample * (rect.height() * Y_SCALE);
    Pos2::new(x, y)
}

fn sample_from_pos(rect: Rect, pos: Pos2) -> (usize, f32) {
    let x_norm = ((pos.x - rect.min.x) / rect.width()).clamp(0.0, 1.0);
    let idx = (x_norm * (WT_SIZE - 1) as f32) as usize;
    let y_norm = ((rect.center().y - pos.y) / (rect.height() * Y_SCALE)).clamp(-1.0, 1.0);
    (idx, y_norm)
}
