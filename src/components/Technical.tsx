import { Copy, Check } from 'lucide-react';
import { useState } from 'react';

function CodeBlock({ code, label }: { code: string; label?: string }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = () => {
    navigator.clipboard.writeText(code);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="relative group rounded-lg overflow-hidden bg-[#1f1b16] border border-black/10 my-4">
      {label && (
        <div className="bg-[#2d2722] px-4 py-2 text-xs font-mono text-emerald-200 border-b border-black/10">
          {label}
        </div>
      )}
      <div className="p-4 font-mono text-sm text-[#f2e8db] overflow-x-auto">
        <pre>{code}</pre>
      </div>
      <button
        onClick={handleCopy}
        className="absolute top-2 right-2 p-2 rounded-md bg-black/40 text-emerald-200 opacity-0 group-hover:opacity-100 transition-opacity hover:text-white hover:bg-black/60"
      >
        {copied ? <Check className="w-4 h-4 text-emerald-500" /> : <Copy className="w-4 h-4" />}
      </button>
    </div>
  );
}

export function Technical() {
  return (
    <section id="quickstart" className="py-24">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid lg:grid-cols-2 gap-16">
          
          {/* Left Column: Quickstart & Prerequisites */}
          <div>
            <p className="text-xs uppercase tracking-[0.3em] text-emerald-600 font-semibold">Quickstart</p>
            <h2 className="text-3xl font-bold text-[#1f1b16] mt-3 mb-8">Build and run locally</h2>
            
            <div className="mb-12">
              <h3 className="text-xl font-semibold text-[#1f1b16] mb-4">1. Run as standalone app</h3>
              <p className="text-[#5b5346] mb-4">Launch the wavetable designer directly on your desktop.</p>
              <CodeBlock 
                label="Terminal"
                code={`cd wavetable_designer
cargo run -p wavetable_designer`}
              />
            </div>

            <div className="mb-12">
              <h3 className="text-xl font-semibold text-[#1f1b16] mb-4">2. Build CLAP/VST3 plugin</h3>
              <p className="text-[#5b5346] mb-4">Bundle the plugin for use in your DAW.</p>
              <CodeBlock 
                label="Terminal"
                code={`cd wavetable_designer
cargo xtask bundle wavetable_designer --release`}
              />
              <p className="text-sm text-[#7a6f61] mt-2">
                Bundles are placed in <code className="text-[#1f1b16]">wavetable_designer/target/bundled/</code>
              </p>
            </div>

            <div className="bg-white/70 rounded-xl p-6 border border-black/10 mb-12 shadow-sm">
              <h3 className="text-lg font-semibold text-[#1f1b16] mb-4">Prerequisites</h3>
              <div className="overflow-x-auto">
                <table className="w-full text-left text-sm">
                  <thead>
                    <tr className="border-b border-black/10 text-[#7a6f61]">
                      <th className="pb-3 font-medium">Tool</th>
                      <th className="pb-3 font-medium">Version</th>
                      <th className="pb-3 font-medium">Notes</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-black/5 text-[#5b5346]">
                    <tr>
                      <td className="py-3 font-mono text-[#1f1b16]">Rust</td>
                      <td className="py-3">1.75+</td>
                      <td className="py-3 font-mono text-[#7a6f61]">rustup update stable</td>
                    </tr>
                    <tr>
                      <td className="py-3 font-mono text-[#1f1b16]">just</td>
                      <td className="py-3">Any</td>
                      <td className="py-3 text-[#7a6f61]">Optional task runner</td>
                    </tr>
                    <tr>
                      <td className="py-3 font-mono text-[#1f1b16]">ALSA</td>
                      <td className="py-3">-</td>
                      <td className="py-3 font-mono text-[#7a6f61]">sudo apt install libasound2-dev</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

          </div>

          {/* Right Column: Dev Workflow & Architecture */}
          <div>
            <p className="text-xs uppercase tracking-[0.3em] text-emerald-600 font-semibold">Workflow</p>
            <h2 className="text-3xl font-bold text-[#1f1b16] mt-3 mb-8">Developer workflow</h2>
            
            <div className="mb-12">
              <p className="text-[#5b5346] mb-6">
                Common development tasks are automated with <code className="text-emerald-700">just</code>.
              </p>
              
              <div className="grid gap-4">
                {[
                  { cmd: "just dev", desc: "Run in standalone mode" },
                  { cmd: "just bundle", desc: "Release bundle (CLAP + VST3)" },
                  { cmd: "just fmt", desc: "Format code with cargo fmt" },
                  { cmd: "just clippy", desc: "Lint with cargo clippy" },
                  { cmd: "just test", desc: "Run unit tests" },
                ].map((task, i) => (
                  <div key={i} className="flex items-center justify-between p-4 bg-white/70 rounded-lg border border-black/10 hover:border-emerald-500/30 transition-colors">
                    <code className="text-emerald-700 font-mono">{task.cmd}</code>
                    <span className="text-[#5b5346] text-sm">{task.desc}</span>
                  </div>
                ))}
              </div>
            </div>

            <div id="architecture" className="bg-white/70 rounded-xl p-8 border border-black/10 mb-12 shadow-sm">
              <h3 className="text-xl font-bold text-[#1f1b16] mb-6">Architecture overview</h3>
              <div className="space-y-6 text-[#5b5346] text-sm leading-relaxed">
                <p>
                  <strong className="text-[#1f1b16]">Core pattern:</strong> WavetableDesigner implements Plugin and owns an <code className="text-emerald-700">Arc&lt;Mutex&lt;WtState&gt;&gt;</code> shared with the editor closure.
                </p>
                <p>
                  <strong className="text-[#1f1b16]">State management:</strong> WtState holds all mutable editor state: frames, active frame index, FM/effect parameters, and preview settings.
                </p>
                <p>
                  <strong className="text-[#1f1b16]">Audio process:</strong> On each buffer, <code className="text-emerald-700">process()</code> reads the current baked frame and runs a phase-accumulator oscillator with linear interpolation.
                </p>
                <p>
                  <strong className="text-[#1f1b16]">UI and DSP:</strong> The egui UI mutates WtState directly and calls <code className="text-emerald-700">bake_wavetable</code> on parameter changes, applying the FM to Boost to Wavefold chain.
                </p>
              </div>
            </div>

            <div className="bg-white/70 rounded-xl p-6 border border-black/10 mb-12 shadow-sm">
              <h3 className="text-lg font-semibold text-[#1f1b16] mb-4">Preview modes</h3>
              <div className="overflow-x-auto">
                <table className="w-full text-left text-sm">
                  <thead>
                    <tr className="border-b border-black/10 text-[#7a6f61]">
                      <th className="pb-3 font-medium w-32">Mode</th>
                      <th className="pb-3 font-medium">Behavior</th>
                    </tr>
                  </thead>
                  <tbody className="divide-y divide-black/5 text-[#5b5346]">
                    <tr>
                      <td className="py-3 font-mono text-emerald-700">Off</td>
                      <td className="py-3">No audio output; phase is held at zero.</td>
                    </tr>
                    <tr>
                      <td className="py-3 font-mono text-emerald-700">Edit-Drone</td>
                      <td className="py-3">Plays selected note continuously while dragging. Hear edits in real time.</td>
                    </tr>
                    <tr>
                      <td className="py-3 font-mono text-emerald-700">MIDI</td>
                      <td className="py-3">Monophonic input - last note wins; velocity controls amplitude.</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>

            <div className="bg-[#1f1b16] border border-black/10 rounded-xl p-6 font-mono text-sm text-[#f2e8db] overflow-x-auto">
              <h3 className="text-lg font-sans font-semibold text-white mb-4">Repository layout</h3>
              <pre className="leading-relaxed">
{`TableStudio/
├── README.md
└── wavetable_designer/
    ├── src/
    │   ├── lib.rs             # Plugin struct, audio loop
    │   ├── main.rs            # Standalone entry point
    │   ├── app_state.rs       # Core data model
    │   ├── dsp.rs             # Bake pipeline, FM/effects
    │   ├── widgets.rs         # Custom synth-knob widget
    │   └── editor/
    │       ├── mod.rs         # UI layout, WAV I/O
    │       ├── canvas.rs      # Waveform draw canvas
    │       ├── grid.rs        # Frame selection grid
    │       └── preview.rs     # Preview mode selector
    ├── docs/
    │   ├── architecture.md
    │   └── accelerators.md
    ├── xtask/                 # NIH-plug bundler
    └── bundler.toml`}
              </pre>
            </div>

          </div>
        </div>
      </div>
    </section>
  );
}
