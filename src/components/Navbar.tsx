import { Github, Menu, X } from 'lucide-react';
import { useState } from 'react';

export function Navbar() {
  const [isOpen, setIsOpen] = useState(false);

  return (
    <nav className="fixed w-full z-50 bg-zinc-950/80 backdrop-blur-md border-b border-white/10">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex items-center justify-between h-16">
          <div className="flex items-center">
            <div className="flex-shrink-0">
              <span className="text-xl font-bold tracking-tight text-white">TableStudio</span>
            </div>
            <div className="hidden md:block">
              <div className="ml-10 flex items-baseline space-x-4">
                <a href="#features" className="text-zinc-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors">Features</a>
                <a href="#quickstart" className="text-zinc-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors">Quickstart</a>
                <a href="#architecture" className="text-zinc-300 hover:text-white px-3 py-2 rounded-md text-sm font-medium transition-colors">Architecture</a>
              </div>
            </div>
          </div>
          <div className="hidden md:block">
            <a
              href="https://github.com/sp80808/TableStudio"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 bg-white text-zinc-900 px-4 py-2 rounded-md text-sm font-medium hover:bg-zinc-200 transition-colors"
            >
              <Github className="w-4 h-4" />
              GitHub
            </a>
          </div>
          <div className="-mr-2 flex md:hidden">
            <button
              onClick={() => setIsOpen(!isOpen)}
              className="inline-flex items-center justify-center p-2 rounded-md text-zinc-400 hover:text-white hover:bg-zinc-800 focus:outline-none"
            >
              {isOpen ? <X className="w-6 h-6" /> : <Menu className="w-6 h-6" />}
            </button>
          </div>
        </div>
      </div>

      {isOpen && (
        <div className="md:hidden bg-zinc-900 border-b border-white/10">
          <div className="px-2 pt-2 pb-3 space-y-1 sm:px-3">
            <a href="#features" className="text-zinc-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium">Features</a>
            <a href="#quickstart" className="text-zinc-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium">Quickstart</a>
            <a href="#architecture" className="text-zinc-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium">Architecture</a>
            <a
              href="https://github.com/sp80808/TableStudio"
              target="_blank"
              rel="noopener noreferrer"
              className="text-zinc-300 hover:text-white block px-3 py-2 rounded-md text-base font-medium flex items-center gap-2"
            >
              <Github className="w-4 h-4" />
              GitHub
            </a>
          </div>
        </div>
      )}
    </nav>
  );
}
