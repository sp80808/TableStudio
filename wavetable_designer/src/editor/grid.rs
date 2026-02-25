use crate::app_state::{WtState, WT_SIZE};
use nih_plug_egui::egui::{self, Color32, Stroke, Vec2};

const PREVIEW_HEIGHT: f32 = 36.0;

pub fn draw_frame_grid(ui: &mut egui::Ui, state: &mut WtState) {
    let rows = state.grid_rows.max(1);
    let cols = state.grid_cols.max(1);

    egui::ScrollArea::vertical().show(ui, |ui| {
        for row in 0..rows {
            ui.horizontal(|ui| {
                for col in 0..cols {
                    let index = row * cols + col;
                    if index >= state.frames.len() {
                        break;
                    }
                    let is_active = index == state.active_frame;
                    let response = draw_frame_preview(ui, &state.frames[index], is_active);
                    if response.clicked() {
                        state.active_frame = index;
                    }
                }
            });
        }
    });
}

fn draw_frame_preview(
    ui: &mut egui::Ui,
    frame: &crate::app_state::WavetableFrame,
    is_active: bool,
) -> egui::Response {
    let size = Vec2::new(80.0, PREVIEW_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());
    let painter = ui.painter_at(rect);

    let bg = if is_active {
        Color32::from_rgb(40, 60, 80)
    } else {
        Color32::from_gray(24)
    };
    painter.rect_filled(rect, 3.0, bg);

    let mut points = Vec::with_capacity(WT_SIZE);
    for (i, &sample) in frame.baked.iter().enumerate() {
        let x = rect.min.x + (i as f32 / WT_SIZE as f32) * rect.width();
        let y = rect.center().y - sample * (rect.height() * 0.45);
        points.push(egui::pos2(x, y));
    }
    painter.add(egui::Shape::line(
        points,
        Stroke::new(1.0, Color32::from_rgb(200, 200, 200)),
    ));

    response
}
