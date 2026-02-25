import { Github, Menu, X } from 'lucide-react';
import { useState } from 'react';

export function Navbar() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <nav className="fixed w-full z-50 bg-[#f6f1e8]/80 backdrop-blur-md border-b border-black/10">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <span className="text-xl font-bold tracking-tight text-[#1f1b16]">TableStudio</span>
            </div>
            <div className="hidden md:block">
              <div className="ml-10 flex items-baseline space-x-4">
                <a href="#downloads" className="text-[#5b5346] hover:text-[#1f1b16] px-3 py-2 rounded-md text-sm font-medium transition-colors">Downloads</a>
                <a href="#features" className="text-[#5b5346] hover:text-[#1f1b16] px-3 py-2 rounded-md text-sm font-medium transition-colors">Features</a>
                <a href="#quickstart" className="text-[#5b5346] hover:text-[#1f1b16] px-3 py-2 rounded-md text-sm font-medium transition-colors">Quickstart</a>
                <a href="#sources" className="text-[#5b5346] hover:text-[#1f1b16] px-3 py-2 rounded-md text-sm font-medium transition-colors">Sources</a>
                <a href="#support" className="text-[#5b5346] hover:text-[#1f1b16] px-3 py-2 rounded-md text-sm font-medium transition-colors">Support</a>
              </div>
            </div>
          </div>
          <div className="hidden md:block">
            <div className="flex items-center gap-3">
              <a
                href="https://github.com/sp80808/TableStudio/releases"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 bg-[#1f1b16] text-[#fdf8f1] px-4 py-2 rounded-full text-sm font-semibold hover:bg-[#2d2722] transition-colors"
              >
                Releases
              </a>
              <a
                href="https://github.com/sp80808/TableStudio"
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 border border-black/10 bg-white/70 text-[#1f1b16] px-4 py-2 rounded-full text-sm font-semibold hover:bg-white transition-colors"
              >
                <Github className="w-4 h-4" />
                GitHub
              </a>
            </div>
          </div>
          <div className="-mr-2 flex md:hidden">
            <button
              onClick={() => setIsOpen(!isOpen)}
              className="inline-flex items-center justify-center p-2 rounded-md text-[#7a6f61] hover:text-[#1f1b16] hover:bg-black/5 focus:outline-none"
            >
              {isOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
            </button>
          </div>
        </div>
      </div>

      {isOpen && (
        <div className="md:hidden bg-[#f6f1e8] border-b border-black/10">
          <div className="px-2 pt-2 pb-3 space-y-1 sm:px-3">
            <a href="#downloads" className="text-[#5b5346] hover:text-[#1f1b16] block px-3 py-2 rounded-md text-base font-medium">Downloads</a>
            <a href="#features" className="text-[#5b5346] hover:text-[#1f1b16] block px-3 py-2 rounded-md text-base font-medium">Features</a>
            <a href="#quickstart" className="text-[#5b5346] hover:text-[#1f1b16] block px-3 py-2 rounded-md text-base font-medium">Quickstart</a>
            <a href="#sources" className="text-[#5b5346] hover:text-[#1f1b16] block px-3 py-2 rounded-md text-base font-medium">Sources</a>
            <a href="#support" className="text-[#5b5346] hover:text-[#1f1b16] block px-3 py-2 rounded-md text-base font-medium">Support</a>
            <a
              href="https://github.com/sp80808/TableStudio"
              target="_blank"
              rel="noopener noreferrer"
              className="text-[#5b5346] hover:text-[#1f1b16] block px-3 py-2 rounded-md text-base font-medium flex items-center gap-2"
            >
              <Github className="w-4 h-4" />
              GitHub
            </a>
            <a
              href="https://github.com/sp80808/TableStudio/releases"
              target="_blank"
              rel="noopener noreferrer"
              className="text-[#5b5346] hover:text-[#1f1b16] block px-3 py-2 rounded-md text-base font-medium"
            >
              Releases
            </a>
          </div>
        </div>
      )}
    </nav>
  );
}
