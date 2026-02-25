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

## Dev Workflow
If you have `just` installed:
```shell
just dev
just bundle
just fmt
just clippy
just test
```

