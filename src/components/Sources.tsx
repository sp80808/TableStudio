import { BookOpen, Github, Layers, Link } from 'lucide-react';

const sources = [
  {
    icon: Github,
    title: 'Source Code',
    description: 'Browse the full TableStudio repository on GitHub.',
    href: 'https://github.com/sp80808/TableStudio',
  },
  {
    icon: BookOpen,
    title: 'Architecture Notes',
    description: 'Dig into the wavetable designer architecture and design notes.',
    href: 'https://github.com/sp80808/TableStudio/blob/master/wavetable_designer/docs/architecture.md',
  },
  {
    icon: Layers,
    title: 'NIH-plug',
    description: 'The Rust audio plugin framework powering TableStudio.',
    href: 'https://github.com/robbert-vdh/nih-plug',
  },
  {
    icon: Link,
    title: 'egui',
    description: 'The immediate mode UI toolkit behind the editor.',
    href: 'https://github.com/emilk/egui',
  },
];

export function Sources() {
  return (
    <section id="sources" className="py-24 bg-[#f1ebe1] border-y border-black/5">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="text-center mb-14">
          <p className="text-xs uppercase tracking-[0.3em] text-emerald-600 font-semibold">Sources</p>
          <h2 className="text-3xl md:text-4xl font-bold text-[#1f1b16] mt-3">Everything behind the build</h2>
          <p className="text-lg text-[#5b5346] mt-4 max-w-2xl mx-auto">
            Follow the source code, architecture notes, and upstream projects that make TableStudio possible.
          </p>
        </div>

        <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-4">
          {sources.map((item) => (
            <a
              key={item.title}
              href={item.href}
              target="_blank"
              rel="noopener noreferrer"
              className="group rounded-2xl border border-black/10 bg-white/70 p-6 shadow-sm hover:shadow-lg transition"
            >
              <div className="w-12 h-12 rounded-xl bg-emerald-100 text-emerald-700 flex items-center justify-center mb-5">
                <item.icon className="w-6 h-6" />
              </div>
              <h3 className="text-lg font-semibold text-[#1f1b16] mb-2 group-hover:text-emerald-700 transition">
                {item.title}
              </h3>
              <p className="text-[#5b5346] leading-relaxed">{item.description}</p>
            </a>
          ))}
        </div>
      </div>
    </section>
  );
}
