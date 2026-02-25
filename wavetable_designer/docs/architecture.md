# Wavetable Designer: Architecture + Development Notes

## Overview
Wavetable Designer is a NIH-plug based wavetable editor with an egui UI. It supports multi-frame wavetable editing, harmonics preview, live audio preview modes, and early-stage spectral tooling (WIP).

## Repository layout
- `src/lib.rs`: Plugin implementation, audio preview, MIDI input handling.
- `src/app_state.rs`: Core data model (frames, preview mode, parameters).
- `src/dsp.rs`: Wavetable bake pipeline + FFT harmonics + note-to-freq helpers.
- `src/editor/`: UI split into canvas, grid, and preview panels.
- `src/main.rs`: Standalone wrapper entry point.
- `xtask/`: NIH-plug bundler task runner.
- `bundler.toml`: Bundler metadata for `cargo xtask bundle`.
- `docs/accelerators.md`: Porting notes and upstream references.

## Current functionality
- Multi-frame wavetable grid (default 8x8), with active frame selection and keyframe markers.
- Dual display of raw + baked waveform, drag-to-edit.
- Harmonics view using FFT with context-menu FFT operations (clear, randomize, even/odd).
- Preview modes: Off / Edit-Drone / MIDI (monophonic).
- WAV import/export for current frame or all frames.
- Spectral controls exposed in UI (WIP) with a processing stub in `dsp.rs`.

## Key data flow
- `WtState` contains frame data and preview settings. `active_frame` selects edit target.
- `editor::draw_ui` mutates state and triggers `bake_wavetable` when needed.
- `process()` reads active frame + preview mode and produces audio.

## Dependencies
- `nih_plug` + `nih_plug_egui`: plugin + GUI
- `hound`: WAV read/write
- `rfd`: native file dialogs
- `parking_lot`: fast mutex for shared state
- `rustfft`: FFT for harmonics view
- `num-complex`: FFT complex math

## Next improvements (optional)
- Add undo/redo for waveform edits.
- Add interpolation between frames for morphing preview.
- Optimize draw perf for very large grids.
