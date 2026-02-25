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
    description: "Default 8×8 grid (64 frames). Click any cell to select and edit it instantly."
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
    <section id="features" className="py-24 bg-zinc-900 border-t border-white/5">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-16">
          <h2 className="text-3xl md:text-4xl font-bold text-white mb-4">Powerful Features</h2>
          <p className="text-zinc-400 max-w-2xl mx-auto">
            Everything you need to design complex, evolving wavetables for your music production.
          </p>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
          {features.map((feature, index) => (
            <div 
              key={index}
              className="bg-zinc-950/50 border border-white/5 p-6 rounded-xl hover:border-emerald-500/30 transition-colors group"
            >
              <div className="w-12 h-12 bg-zinc-900 rounded-lg flex items-center justify-center mb-4 group-hover:bg-emerald-500/10 transition-colors">
                <feature.icon className="w-6 h-6 text-emerald-500" />
              </div>
              <h3 className="text-lg font-semibold text-white mb-2">{feature.title}</h3>
              <p className="text-zinc-400 text-sm leading-relaxed">
                {feature.description}
              </p>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
