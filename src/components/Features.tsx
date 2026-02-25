import { 
  Grid, 
  PenTool, 
  Activity, 
  Layers, 
  Zap, 
  FileAudio, 
  Music, 
  Settings 
} from 'lucide-react';

const features = [
  {
    icon: Grid,
    title: "Multi-frame Grid",
    description: "Default 8x8 grid (64 frames). Click any cell to select and edit it instantly."
  },
  {
    icon: PenTool,
    title: "Drag-to-Edit Canvas",
    description: "Draw waveforms directly with the mouse. View raw and baked waveforms simultaneously."
  },
  {
    icon: Activity,
    title: "Harmonics View",
    description: "Real-time FFT magnitude display of the active baked frame for precise spectral editing."
  },
  {
    icon: Layers,
    title: "FM Stacking",
    description: "2-op FM pipeline with sine/saw/square mod shapes. Bake non-destructively or commit to raw."
  },
  {
    icon: Zap,
    title: "BassForge Panel",
    description: "Apply fundamental boost and wavefold effects during the bake pass for richer timbres."
  },
  {
    icon: FileAudio,
    title: "WAV Import/Export",
    description: "Load WAVs to populate frames. Export current frame or packed wavetables. Drag-and-drop supported."
  },
  {
    icon: Music,
    title: "Live Audio Preview",
    description: "Three modes: Off, Edit-Drone (plays while dragging), and MIDI (monophonic)."
  },
  {
    icon: Settings,
    title: "Synth Knob Widget",
    description: "Custom egui knob with drag-to-adjust, double-click reset, and arc indicator."
  }
];

export function Features() {
  return (
    <section id="features" className="py-24 bg-[#f1ebe1] border-y border-black/5">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-16">
          <p className="text-xs uppercase tracking-[0.3em] text-emerald-600 font-semibold">Features</p>
          <h2 className="text-3xl md:text-4xl font-bold text-[#1f1b16] mt-3">Built for shaping complex motion</h2>
          <p className="text-[#5b5346] max-w-2xl mx-auto mt-4">
            Every tool is tuned for fast experimentation, clean iteration, and confident export to your DAW.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {features.map((feature, index) => (
            <div
              key={index}
              className="bg-white/70 border border-black/10 p-6 rounded-2xl shadow-sm hover:shadow-lg transition group"
            >
              <div className="w-12 h-12 bg-emerald-100 rounded-xl flex items-center justify-center mb-4 group-hover:bg-emerald-200 transition-colors">
                <feature.icon className="w-6 h-6 text-emerald-700" />
              </div>
              <h3 className="text-lg font-semibold text-[#1f1b16] mb-2">{feature.title}</h3>
              <p className="text-[#5b5346] text-sm leading-relaxed">
                {feature.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
