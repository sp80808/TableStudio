//! Waveform draw canvas and harmonics bar-chart view.
//!
//! The canvas shows the active frame's raw (blue) and baked (orange) waveforms
//! side-by-side.  Dragging on the canvas writes new sample values to
//! `frame.raw` with light neighbour smoothing, and sets `WtState::edit_gate`
//! so the Edit-Drone preview sounds while the mouse button is held.

use crate::app_state::{WtState, WT_SIZE};
use crate::dsp::compute_harmonics;
use nih_plug_egui::egui::{self, Color32, Pos2, Rect, Sense, Stroke, Vec2};

const Y_SCALE: f32 = 0.45;
const GRID_DIVS: usize = 8;
const HARMONICS_BINS: usize = 128;

/// Draws the waveform canvas and harmonics view, handles drag-to-edit input,
/// and returns `true` if the raw waveform was modified (indicating a bake is
/// required).
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

        let mut baked_points = Vec::with_capacity(WT_SIZE);
        for (i, &sample) in frame.baked.iter().enumerate() {
            baked_points.push(sample_to_pos(rect, i, sample));
        }
        painter.add(egui::Shape::line(
            baked_points,
            Stroke::new(3.0, Color32::from_rgb(255, 140, 0)),
        ));

        let mut raw_points = Vec::with_capacity(WT_SIZE);
        for (i, &sample) in frame.raw.iter().enumerate() {
            raw_points.push(sample_to_pos(rect, i, sample));
        }
        painter.add(egui::Shape::line(
            raw_points,
            Stroke::new(1.5, Color32::from_rgb(50, 150, 255)),
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
    draw_harmonics(ui, state.active_frame());

    needs_bake
}

fn draw_harmonics(ui: &mut egui::Ui, frame: &crate::app_state::WavetableFrame) {
    ui.label("Harmonics");
    let harmonics = compute_harmonics(&frame.baked);
    let bins = harmonics.len().min(HARMONICS_BINS);

    let (rect, _) = ui.allocate_exact_size(Vec2::new(ui.available_width(), 120.0), Sense::hover());
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 4.0, Color32::from_gray(24));

    let max_val = harmonics
        .iter()
        .take(bins)
        .fold(0.0_f32, |acc, &v| acc.max(v));
    let max_val = if max_val > 0.0 { max_val } else { 1.0 };

    if bins <= 1 {
        return;
    }

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
