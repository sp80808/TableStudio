//! Preview mode selector, note picker, and gain slider panel.
//!
//! Allows the user to switch between Off / Edit-Drone / MIDI preview modes,
//! choose the drone note and detune offset, and set the master preview gain
//! via the NIH-plug parameter slider.

use crate::app_state::{PreviewMode, WtState};
use crate::dsp::note_to_freq;
use crate::WtParams;
use nih_plug::prelude::ParamSetter;
use nih_plug_egui::egui::{self, DragValue};

const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

/// Renders the Preview section: mode combo-box, note picker (Edit-Drone only),
/// detune drag-value, current frequency readout, and the Preview Gain slider.
pub fn draw_preview_panel(
    ui: &mut egui::Ui,
    state: &mut WtState,
    params: &std::sync::Arc<WtParams>,
    setter: &ParamSetter,
) {
    ui.heading("Preview");

    ui.horizontal(|ui| {
        ui.label("Mode:");
        egui::ComboBox::from_id_source("preview_mode")
            .selected_text(state.preview_mode.label())
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut state.preview_mode, PreviewMode::Off, "Off");
                ui.selectable_value(&mut state.preview_mode, PreviewMode::EditDrone, "Edit-Drone");
                ui.selectable_value(&mut state.preview_mode, PreviewMode::Midi, "MIDI");
            });
    });

    if state.preview_mode == PreviewMode::EditDrone {
        ui.horizontal(|ui| {
            ui.label("Note:");
            note_picker(ui, &mut state.preview_note);
            ui.add(DragValue::new(&mut state.preview_detune_cents)
                .clamp_range(-50.0..=50.0)
                .speed(0.5)
                .suffix(" cents"));
        });

        let freq = note_to_freq(state.preview_note, state.preview_detune_cents);
        ui.label(format!("Frequency: {:.2} Hz", freq));
        ui.label("Plays only while dragging on the canvas.");
    } else if state.preview_mode == PreviewMode::Midi {
        ui.label("MIDI input active (monophonic). Last note wins.");
    }

    ui.add(nih_plug_egui::widgets::ParamSlider::for_param(
        &params.preview_gain,
        setter,
    ));
}

fn note_picker(ui: &mut egui::Ui, note: &mut u8) {
    let octave = (*note as i32 / 12) - 1; // MIDI note 0 = C-1
    let note_index = (*note as usize) % 12;

    egui::ComboBox::from_id_source("preview_note_name")
        .selected_text(format!("{}{}", NOTE_NAMES[note_index], octave))
        .show_ui(ui, |ui| {
            for octave in -1..=8 {
                for (idx, name) in NOTE_NAMES.iter().enumerate() {
                    let midi_note = (octave + 1) * 12 + idx as i32;
                    if !(0..=127).contains(&midi_note) {
                        continue;
                    }
                    let label = format!("{}{}", name, octave);
                    ui.selectable_value(note, midi_note as u8, label);
                }
            }
        });
}
