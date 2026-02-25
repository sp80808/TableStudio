import { HardDriveDownload, Monitor, Laptop } from 'lucide-react';

const downloads = [
  {
    icon: HardDriveDownload,
    title: 'Latest Release',
    description: 'Grab the newest build for macOS, Windows, and Linux from GitHub Releases.',
    href: 'https://github.com/sp80808/TableStudio/releases',
    cta: 'Go to Releases',
  },
  {
    icon: Monitor,
    title: 'Release Notes',
    description: 'Track progress, changelogs, and previous builds in the release feed.',
    href: 'https://github.com/sp80808/TableStudio/releases',
    cta: 'View Notes',
  },
  {
    icon: Laptop,
    title: 'Build from Source',
    description: 'Prefer to compile locally? Follow the repo quickstart and build steps.',
    href: 'https://github.com/sp80808/TableStudio#quickstart',
    cta: 'Read Quickstart',
  },
];

export function Downloads() {
  return (
    <section id="downloads" className="py-24">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex flex-col lg:flex-row lg:items-end lg:justify-between gap-6 mb-12">
          <div>
            <p className="text-xs uppercase tracking-[0.3em] text-emerald-600 font-semibold">
              Downloads
            </p>
            <h2 className="text-3xl md:text-4xl font-bold text-[#1f1b16] mt-3">
              Releases are coming soon
            </h2>
            <p className="text-lg text-[#5b5346] mt-4 max-w-2xl">
              The first public build is almost ready. Until then, the Releases page will host all installers,
              changelogs, and checksums.
            </p>
          </div>
          <a
            href="https://github.com/sp80808/TableStudio/releases"
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center justify-center px-6 py-3 rounded-full bg-[#1f1b16] text-[#fdf8f1] font-semibold shadow-lg shadow-black/10 hover:bg-[#2d2722] transition"
          >
            Visit GitHub Releases
          </a>
        </div>

        <div className="grid gap-6 md:grid-cols-3">
          {downloads.map((item) => (
            <div
              key={item.title}
              className="h-full rounded-2xl border border-black/10 bg-white/70 p-6 shadow-sm hover:shadow-lg transition"
            >
              <div className="w-12 h-12 rounded-xl bg-emerald-100 text-emerald-700 flex items-center justify-center mb-5">
                <item.icon className="w-6 h-6" />
              </div>
              <h3 className="text-xl font-semibold text-[#1f1b16] mb-2">{item.title}</h3>
              <p className="text-[#5b5346] mb-6 leading-relaxed">{item.description}</p>
              <a
                href={item.href}
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center text-sm font-semibold text-emerald-700 hover:text-emerald-900"
              >
                {item.cta}
              </a>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
