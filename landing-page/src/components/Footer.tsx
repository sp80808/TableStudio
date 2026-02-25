export function Footer() {
  return (
    <footer className="bg-zinc-950 border-t border-white/5 py-12">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col md:flex-row justify-between items-center gap-6">
        <div>
          <span className="text-xl font-bold tracking-tight text-white">TableStudio</span>
          <p className="text-zinc-500 text-sm mt-2">
            Open-source wavetable workstation.
          </p>
        </div>
        
        <div className="flex gap-8 text-sm text-zinc-400">
          <a href="#" className="hover:text-white transition-colors">Documentation</a>
          <a href="https://github.com/sp80808/TableStudio" target="_blank" rel="noopener noreferrer" className="hover:text-white transition-colors">GitHub</a>
          <a href="#" className="hover:text-white transition-colors">License</a>
        </div>

        <div className="text-zinc-600 text-sm">
          MIT License &copy; {new Date().getFullYear()}
        </div>
      </div>
    </footer>
  );
}
