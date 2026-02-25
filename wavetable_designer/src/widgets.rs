use nih_plug_egui::egui::{self, Color32, Response, Sense, Stroke, Ui, Vec2};

/// A premium, well-documented Synth Knob widget in the style of Serum/Vital.
/// Perfect for BassForge and FM Injector controls.
pub fn synth_knob(
    ui: &mut Ui,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    label: &str,
) -> Response {
    let size = 48.0;
    
    // We allocate a vertical layout for the knob and its texts
    ui.vertical_centered(|ui| {
        let (rect, mut response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click_and_drag());
        
        // Handle Dragging
        if response.dragged() {
            let delta = response.drag_delta();
            let drag_amount = -delta.y; // Dragging up increases value
            
            let range_span = range.end() - range.start();
            let change = (drag_amount / 150.0) * range_span;
            *value = (*value + change).clamp(*range.start(), *range.end());
            response.mark_changed();
        }
        
        // Handle Double Click to reset
        if response.double_clicked() {
            if range.contains(&0.0) {
                *value = 0.0;
            } else {
                *value = *range.start();
            }
            response.mark_changed();
        }

        // --- Drawing ---
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = rect.center();
            let radius = size / 2.0;
            
            // Normalized value (0.0 to 1.0)
            let norm_val = (*value - range.start()) / (range.end() - range.start());
            
            // Angles for the arc (start bottom left, end bottom right)
            let angle_start = std::f32::consts::PI * 0.75;
            let angle_end = std::f32::consts::PI * 2.25;
            let angle_current = angle_start + (angle_end - angle_start) * norm_val;
            
            let hover_glow = if response.dragged() || response.hovered() {
                Color32::from_rgb(255, 140, 0)
            } else {
                Color32::from_rgb(200, 100, 0)
            };

            // Outer ring (dark background)
            painter.circle_stroke(center, radius * 0.85, Stroke::new(4.0, Color32::from_gray(40)));
            
            // Fill arc (approximate with line for now, a proper egui shape arc is ideal)
            // But we can approximate it by drawing an indicator
            let mut track_points = vec![];
            let steps = 24;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let a = angle_start + (angle_current - angle_start) * t;
                track_points.push(egui::pos2(
                    center.x + (radius * 0.85) * a.cos(),
                    center.y + (radius * 0.85) * a.sin(),
                ));
            }
            if track_points.len() > 1 {
                painter.add(egui::Shape::line(track_points, Stroke::new(4.0, hover_glow)));
            }

            // Inner Knob body
            painter.circle_filled(center, radius * 0.65, Color32::from_gray(30));
            painter.circle_stroke(center, radius * 0.65, Stroke::new(1.0, Color32::from_gray(60)));
            
            // Indicator tick
            let p1 = egui::pos2(
                center.x + (radius * 0.3) * angle_current.cos(),
                center.y + (radius * 0.3) * angle_current.sin()
            );
            let p2 = egui::pos2(
                center.x + (radius * 0.6) * angle_current.cos(),
                center.y + (radius * 0.6) * angle_current.sin()
            );
            painter.line_segment([p1, p2], Stroke::new(2.5, Color32::WHITE));
        }
        
        // Text labels
        ui.label(egui::RichText::new(label).color(Color32::LIGHT_GRAY).size(11.0));
        ui.label(egui::RichText::new(format!("{:.2}", value)).color(Color32::from_gray(140)).size(10.0));
        
        response
    }).inner
}
