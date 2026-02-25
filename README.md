# TableStudio

**TableStudio** is an open-source wavetable workstation. The initial component is the **Wavetable Designer** — a multi-frame wavetable editor and synthesizer plugin built on [NIH-plug](https://github.com/robbert-vdh/nih-plug) and [egui](https://github.com/emilk/egui).

---

## Features

- **Multi-frame wavetable grid** — default 8×8 grid (64 frames); click any cell to select and edit it.
- **Drag-to-edit canvas** — draw waveforms directly with the mouse; raw and baked waveforms are shown simultaneously.
- **Harmonics view** — real-time FFT magnitude display of the active baked frame.
- **FM Stacking pipeline** — 2-op FM (carrier + modulator) with sine/saw/square mod shapes, baked non-destructively or committed to the raw waveform.
- **BassForge panel** — fundamental boost and wavefold effects applied during the bake pass.
- **WAV import/export** — load a WAV to populate one or all frames; export the current frame or all frames as a packed wavetable WAV. Drag-and-drop is also supported.
- **Live audio preview** — three modes: Off, Edit-Drone (sound plays while dragging), and MIDI (monophonic, last-note-wins).
- **Synth knob widget** — custom egui knob with drag-to-adjust, double-click reset, and arc indicator.
- **CLAP + VST3 plugin** — ships as both a CLAP and VST3 instrument plugin via NIH-plug.

---

## Prerequisites

| Tool | Version | Notes |
|------|---------|-------|
| [Rust](https://rustup.rs/) | 1.75+ (stable) | `rustup update stable` |
| [just](https://github.com/casey/just) | any | Optional; shorthand for common tasks |
| ALSA headers | — | Linux only: `sudo apt install libasound2-dev` |
| Xorg/Wayland dev libs | — | Linux only: `sudo apt install libx11-dev libxcb1-dev` |

---

## Quickstart

### Run as a standalone desktop app

```shell
cd wavetable_designer
cargo run -p wavetable_designer
```

### Build and bundle as a CLAP/VST3 plugin

```shell
cd wavetable_designer
cargo xtask bundle wavetable_designer --release
```

Bundles are placed in `wavetable_designer/target/bundled/`.

---

## Dev Workflow

If you have [`just`](https://github.com/casey/just) installed, all common tasks are one command away:

| Command | Description |
|---------|-------------|
| `just dev` | Run in standalone mode |
| `just bundle` | Release bundle (CLAP + VST3) |
| `just fmt` | Format code with `cargo fmt` |
| `just clippy` | Lint with `cargo clippy` |
| `just test` | Run unit tests |

---

## Preview Modes

| Mode | Behaviour |
|------|-----------|
| **Off** | No audio output; phase is held at zero |
| **Edit-Drone** | Plays the selected note continuously **while you drag** on the canvas. Useful for hearing waveform edits in real time. |
| **MIDI** | Monophonic input — last note wins; velocity controls amplitude |

---

## Repository Layout

```
TableStudio/
├── README.md                  ← this file
└── wavetable_designer/
    ├── src/
    │   ├── lib.rs             ← plugin struct, audio process loop, NIH-plug glue
    │   ├── main.rs            ← standalone entry point
    │   ├── app_state.rs       ← core data model (WtState, WavetableFrame, PreviewMode)
    │   ├── dsp.rs             ← bake pipeline, FFT helpers, FM/wavefold effects
    │   ├── widgets.rs         ← custom synth-knob egui widget
    │   └── editor/
    │       ├── mod.rs         ← top-level UI layout, WAV I/O, file-drop handling
    │       ├── canvas.rs      ← waveform draw canvas + harmonics view
    │       ├── grid.rs        ← frame selection grid panel
    │       └── preview.rs     ← preview mode selector and note picker
    ├── docs/
    │   ├── architecture.md    ← architecture and development notes
    │   └── accelerators.md    ← open-source reference notes
    ├── xtask/                 ← NIH-plug bundler task runner
    ├── bundler.toml           ← plugin metadata for cargo xtask bundle
    └── justfile               ← just task runner shortcuts
```

---

## Architecture Overview

The plugin follows the standard NIH-plug pattern:

1. **`WavetableDesigner`** implements `Plugin` and owns an `Arc<Mutex<WtState>>` shared with the editor closure.
2. **`WtState`** holds all mutable editor state: frames, active frame index, FM/effect parameters, and preview settings.
3. On each audio buffer, `process()` reads the current baked frame and preview mode, then runs a phase-accumulator oscillator (`sample_from_table`) with linear interpolation.
4. The egui UI (`editor::draw_ui`) mutates `WtState` directly (under the mutex) and calls `bake_wavetable` whenever any parameter changes.
5. `bake_wavetable` applies the FM → fundamental-boost → wavefold chain to the raw samples and writes the result to `frame.baked`.

See [`docs/architecture.md`](wavetable_designer/docs/architecture.md) for a more detailed breakdown.

---

## License

MIT — see [`wavetable_designer/Cargo.toml`](wavetable_designer/Cargo.toml) for the declared license.  
Note: `nih_plug` and its egui integration are MIT-licensed upstream.

