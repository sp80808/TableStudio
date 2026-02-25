# TableStudio Wavetable Designer

A powerful, cross-platform wavetable designer and editor built with Rust and the [`nih-plug`](https://github.com/robbert-vdh/nih-plug) framework. Designed from the ground up to operate efficiently both as a standalone desktop application and as a VST3 / CLAP audio plugin for your favorite DAW.

## 🚀 Features (Planned & Implemented)
- **Advanced Wavetable Editing:** Design complex wavetables with high precision.
- **Cross-Platform Compatibility:** Works on macOS, Windows, and Linux.
- **Plugin & Standalone:** Runs natively in any VST3/CLAP compatible DAW, or independently as a standalone executable.
- **Hardware-Accelerated UI:** Utilizing [`nih_plug_egui`](https://github.com/robbert-vdh/nih-plug/tree/master/nih_plug_egui) for a smooth, responsive graphical interface.

## 📋 Prerequisites

To compile and run this project, you will need to have **Rust** installed on your system. If you haven't installed it yet, the easiest way is via [rustup.rs](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

You will also need standard C++ build tools for your operating system to compile some dependencies:
- **macOS:** Xcode Command Line Tools (`xcode-select --install`)
- **Windows:** Visual Studio Build Tools with C++ workload
- **Linux:** GCC or Clang (`build-essential` on Ubuntu/Debian)

## 🛠️ Building & Running

### 1. Standalone Application
For testing and development outside of a DAW, you can run the application in standalone mode using Cargo:

```bash
# Run in debug mode (faster compilation, slower execution)
cargo run --bin wavetable_designer

# Run in release mode (slower compilation, maximum performance)
cargo run --release --bin wavetable_designer
```

### 2. Audio Plugin (VST3 / CLAP)
To build the DAW plugins, we utilize the `xtask` bundler provided by `nih-plug`. This will build and package the plugins into their respective formats (`.vst3`, `.clap`, etc.) and place them inside `target/bundled/wavetable_designer/`.

```bash
# Bundle as a release build
cargo xtask bundle wavetable_designer --release
```

**Tip for Development:**
If you want to automatically install the bundled plugins to your local system plugin directories so your DAW can pick them up immediately, use the `--install` flag (or `install` task, depending on xtask configuration):

```bash
cargo xtask install wavetable_designer --release
```

## 📂 Project Structure

- `src/main.rs`: The entry point for the standalone desktop application.
- `src/lib.rs`: The core DSP and plugin interface logic.
- `Cargo.toml`: The Rust project package manafest.
- `xtask/`: The build bundle task runner for nih-plug plugins.

## 📜 License

This project is licensed under the [GPL-3.0-or-later](LICENSE) License. See the `Cargo.toml` file for more details.
