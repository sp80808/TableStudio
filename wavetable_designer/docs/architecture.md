# Wavetable Designer: Architecture + Development Plan

## Overview
The project is a NIH-plug based wavetable editor with an egui UI. It currently focuses on a single 2048-sample frame, with direct canvas editing, quick waveform generators, and WAV import/export. Audio processing is not implemented yet; the plugin is effectively an editor-only tool.

## Repository layout
- `src/lib.rs`: Main plugin implementation, egui editor, canvas drawing, import/export logic, waveform generators.
- `src/main.rs`: Standalone wrapper entry point.
- `src/app_state.rs`: `WtState` data model for future DSP features (currently unused).
- `xtask/`: NIH-plug bundler task runner.
- `bundler.toml`: Bundler metadata for `cargo xtask bundle`.
- `docs/accelerators.md`: Notes on porting patterns from external projects.

## Current functionality
- Drag-to-edit waveform canvas (with line interpolation and neighbor smoothing).
- Waveform generators: Sine, Saw, Square, Triangle, Noise.
- Import/export WAV (mono; multi-channel input takes channel 0).
- Drag-and-drop file overlay + dropped file ingest.
- Egui `ResizableWindow` + persistent `EguiState`.

## Key data flow
- `WavetableDesigner` holds `table: Arc<Mutex<Vec<f32>>>`.
- UI writes to the table inside `editor()` on drag events.
- Import/export operates on the same table and uses `hound` for WAV IO.

## Dependencies
- `nih_plug` + `nih_plug_egui`: plugin + GUI.
- `hound`: WAV read/write.
- `rfd`: native file dialogs.
- `parking_lot`: fast mutex for shared state.

## Gaps / risks
- `src/app_state.rs` is unused and duplicates `WT_SIZE`; likely stale or needs integration.
- No audio preview engine; editor does not generate sound.
- WAV resampling assumes non-periodic source; resulting tables may not be seamless.
- `wt_pos` param is UI-only and not used for anything else.
- No undo/redo or smoothing controls.
- License in `Cargo.toml` is `GPL-3.0-or-later`, which conflicts with MIT/Apache target.

## Continued development plan

### Phase 1: Architecture cleanup
- Decide whether `WtState` becomes the single source of truth. If yes, move table edits into it and remove duplicate `WT_SIZE`.
- Add a `Wavetable` struct (table + metadata + edit history) to centralize operations.
- Update license metadata to match MIT/Apache if required.

### Phase 2: Editing UX improvements
- Add undo/redo stacks with snapshot compression or delta encoding.
- Add smoothing modes (none, linear, gaussian, spline) and adjustable radius/strength.
- Add snap-to-grid and symmetry editing options.
- Improve resampling to enforce seamless loops (wrap-aware interpolation).

### Phase 3: Multi-frame wavetable
- Implement a wavetable grid (N frames) and selection controls.
- Add harmonics view (FFT) and per-frame preview.
- Add interpolation between adjacent frames.

### Phase 4: Audio preview
- Add a phase-accumulator oscillator with linear interpolation.
- Use cpal for real-time preview (play/stop, frequency slider, gain).
- Ensure no allocations in the audio callback; copy shared data into a lock-free buffer.

### Phase 5: File + preset ecosystem
- Support multi-frame WAV import/export with frame slicing.
- Add preset format (JSON/TOML) to save full editor state.
- Add a lightweight file browser panel or custom dialog if native dialogs are insufficient.

### Phase 6: Polishing
- Add parameter automation mapping as features evolve.
- Add theme + color customization.
- Add viewport options (always-on-top, detachable viewports) for standalone use.

## Immediate next actions
- Clarify licensing goal (MIT/Apache vs GPL) and update `Cargo.toml` accordingly.
- Decide whether `WtState` should be integrated or removed.
- Pick priority: audio preview vs multi-frame wavetable grid.
