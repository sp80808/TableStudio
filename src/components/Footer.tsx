export function Footer() {
  return (
    <footer className="border-t border-black/10 py-12 bg-[#f6f1e8]">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex flex-col md:flex-row justify-between items-center gap-6">
        <div>
          <span className="text-xl font-bold tracking-tight text-[#1f1b16]">TableStudio</span>
          <p className="text-[#7a6f61] text-sm mt-2">
            Open-source wavetable workstation.
          </p>
        </div>
        
        <div className="flex flex-wrap justify-center gap-6 text-sm text-[#5b5346]">
          <a href="https://github.com/sp80808/TableStudio/releases" target="_blank" rel="noopener noreferrer" className="hover:text-[#1f1b16] transition-colors">Releases</a>
          <a href="https://github.com/sp80808/TableStudio" target="_blank" rel="noopener noreferrer" className="hover:text-[#1f1b16] transition-colors">Source</a>
          <a href="https://github.com/sp80808/TableStudio/blob/master/wavetable_designer/docs/architecture.md" target="_blank" rel="noopener noreferrer" className="hover:text-[#1f1b16] transition-colors">Architecture</a>
          <a href="#support" className="hover:text-[#1f1b16] transition-colors">Support</a>
        </div>

        <div className="text-[#7a6f61] text-sm">
          MIT License &copy; {new Date().getFullYear()}
        </div>
      </div>
    </footer>
  );
}
