# Wavetable Designer

TableStudio's wavetable editor built on NIH-plug + egui.

## Run (standalone)
```shell
cargo run -p wavetable_designer
```

## Bundle (plugin)
```shell
cargo xtask bundle wavetable_designer --release
```

## Preview Modes
- **Off**: no audio output
- **Edit-Drone**: plays a chosen note **only while dragging** in the editor
- **MIDI**: monophonic input, last-note wins, velocity controls amplitude

## Wavetable Editing
- Multi-frame grid (8x8) with active frame selection and keyframe markers.
- Harmonics view with FFT operations via context menu (clear/randomize/even/odd).
- Spectral controls are exposed as WIP (UI only, evolving DSP).

## Dev Workflow
If you have `just` installed:
```shell
just dev
just bundle
just fmt
just clippy
just test
```
