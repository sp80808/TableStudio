# Open-Source Accelerators: Porting Notes

## Scope and licensing
- Prefer MIT/Apache-compatible patterns.
- GPL or unknown licenses are learning-only. Do not copy code; adapt concepts.
- The current `Cargo.toml` lists `GPL-3.0-or-later`. If the project must be MIT/Apache, update the crate license and verify any dependencies.

## antisvin/owl_wave
Relevant files:
- `src/app.rs`
- `src/grid.rs`
- `src/wave.rs`

Patterns to adapt:
- Wavetable handling and harmonics flow (grid of tables, FFT-derived views).
- File open UI with `rfd::FileDialog`.
- Drag-and-drop file handling in egui (overlay + drop processing).

Port status:
- Import/export actions and drag-and-drop flow were added in `src/lib.rs` (file dialog + file drop overlay).

## ardura/Actuate (GPL-3.0)
Relevant files:
- `src/lib.rs` (Params + plugin struct organization)
- `src/actuate_gui.rs` (editor layout patterns)
- `src/actuate_load_save_dialog.rs` (custom file dialog UI)

Patterns to adapt (learning-only):
- File dialog UX and directory browsing logic.
- UI organization and parameter tooling.
- Preset load/save structure.

Port status:
- Not yet ported. Use as design reference only.

## ardura/Scrollscope
Relevant files:
- `src/scrollscope_gui.rs`

Patterns to adapt:
- High-performance waveform rendering with `ui.painter().extend(shapes)`.
- Batch creation of line shapes from sampled buffers.
- Plot-based oscilloscope view for large buffers.

Port status:
- Not yet ported. Current drawing uses a single polyline; consider batching lines for multi-frame views.

## NIH-plug core examples
Relevant files:
- `plugins/examples/gain_gui_egui/src/lib.rs`

Patterns to adapt:
- `create_egui_editor` + `ResizableWindow` usage.
- `EguiState` persistence.
- Standalone wrapper pattern.

Port status:
- Already aligned with `create_egui_editor` and `ResizableWindow` in `src/lib.rs`.

## emilk/egui demos
Relevant files:
- `crates/egui_demo_lib/src/demo/painting.rs`
- `examples/file_dialog/src/main.rs`

Patterns to adapt:
- `allocate_painter` + freehand drawing (drag -> painter -> Shape::line).
- File drop overlay and `ctx.input().raw.dropped_files` processing.

Port status:
- Drawing uses `allocate_painter` with drag-to-edit and line interpolation in `src/lib.rs`.
- File drop overlay and drop processing are wired in `src/lib.rs`.

## TheWolfSound wavetable tutorial
Patterns to adapt:
- Phase-accumulator oscillator with linear interpolation for table lookup.
- Good baseline for cpal-backed audio preview.

Port status:
- Not yet implemented. Add a preview engine using a phase accumulator and interpolated lookup.

## Current port summary (this repo)
- Canvas allocation + drag-to-edit waveform, interpolated line drawing, and neighbor smoothing.
- Waveform generators (sine/saw/square/triangle/noise).
- Import/export WAV with file dialog plus drag-and-drop overlay.

## Next ports
- Multi-frame wavetable grid and harmonics view (owl_wave patterns).
- Real-time oscilloscope rendering improvements (Scrollscope patterns).
- Preset management + file browser UX (Actuate patterns).
- Live preview engine with phase-accumulator oscillator (TheWolfSound tutorial).
