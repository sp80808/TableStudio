# Wavetable Designer

TableStudio's wavetable editor — a CLAP/VST3 instrument plugin and standalone app built on [NIH-plug](https://github.com/robbert-vdh/nih-plug) + [egui](https://github.com/emilk/egui).

---

## Run (standalone)

```shell
cargo run -p wavetable_designer
```

## Bundle (plugin)

```shell
cargo xtask bundle wavetable_designer --release
```

Bundles are written to `target/bundled/`.

---

## UI Overview

### Frame Grid (left panel)
A scrollable 8×8 grid of miniature waveform previews. Click a cell to make it the **active frame** for editing.

### Canvas (centre, top)
- **Orange waveform** — baked output (after FM / effects chain).
- **Blue waveform** — raw drawn samples.
- **Drag** anywhere on the canvas to paint new sample values. Neighbours are lightly smoothed to avoid sharp discontinuities.

### Harmonics View (centre, below canvas)
Real-time FFT magnitude bar display of the active baked frame (first 128 bins).

### FM Stacking Pipeline
| Control | Range | Description |
|---------|-------|-------------|
| Amount | 0–10 | Depth of frequency modulation applied to the carrier |
| Ratio | 0.1–16 | Modulator frequency as a multiple of the carrier frequency |
| Mod Shape | Sine/Saw/Square | Shape of the modulator waveform |
| Bake FM into Core | button | Commits the baked waveform back to raw (destructive) and resets Amount to 0 |

### BassForge Panel
| Control | Range | Description |
|---------|-------|-------------|
| Fund. Boost | 0–2 | Additively mixes a sine wave at the fundamental to reinforce the low end |
| Wavefold | 0–1 | Applies sine-based wavefolding (`sin(sample × (1 + amount × 4))`) |

### Preview Panel
| Mode | Behaviour |
|------|-----------|
| Off | Silent; phase held at zero |
| Edit-Drone | Plays the selected MIDI note while the mouse button is held on the canvas |
| MIDI | Monophonic; last-note-wins; velocity controls amplitude |

- **Note picker** — selects the Edit-Drone note (C-1 … G8).
- **Detune** — fine-tunes the drone note in cents (±50 ¢).
- **Preview Gain** slider — master output level in dB (−60 … 0 dB).

---

## WAV Import / Export

| Action | Behaviour |
|--------|-----------|
| **Import WAV** | Opens a file picker. If the file contains ≥ 2048 samples, each consecutive 2048-sample block is loaded into a separate frame. Otherwise the file is resampled to 2048 samples and loaded into the active frame. |
| **Export Current** | Saves the active frame's baked waveform as a 32-bit float mono WAV at 44 100 Hz. |
| **Export All** | Concatenates all frame baked waveforms and saves as a single WAV (useful for wavetable-aware samplers). |
| **Drag & drop** | Drag a WAV file onto the plugin window — identical behaviour to Import WAV. |

---

## Preview Modes

- **Off**: no audio output
- **Edit-Drone**: plays a chosen note **only while dragging** in the editor
- **MIDI**: monophonic input, last-note wins, velocity controls amplitude

---

## Wavetable Editing
- Multi-frame grid (8x8) with active frame selection and keyframe markers.
- Harmonics view with FFT operations via context menu (clear/randomize/even/odd).
- Spectral controls are exposed as WIP (UI only, evolving DSP).

## Dev Workflow

If you have [`just`](https://github.com/casey/just) installed:

```shell
just dev      # cargo run -p wavetable_designer
just bundle   # cargo xtask bundle wavetable_designer --release
just fmt      # cargo fmt
just clippy   # cargo clippy --all-targets
just test     # cargo test
```

---

## Further Reading

- [`docs/architecture.md`](docs/architecture.md) — detailed architecture and data-flow notes.
- [`docs/accelerators.md`](docs/accelerators.md) — open-source reference projects and porting notes.
