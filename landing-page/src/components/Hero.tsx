import { motion } from 'motion/react';
import { ArrowRight, Download, Terminal } from 'lucide-react';
import { useEffect, useRef } from 'react';

function WavetableVisualizer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    let animationFrameId: number;
    let time = 0;

    const resize = () => {
      canvas.width = canvas.parentElement?.clientWidth || window.innerWidth;
      canvas.height = canvas.parentElement?.clientHeight || window.innerHeight;
    };

    window.addEventListener('resize', resize);
    resize();

    const draw = () => {
      time += 0.01;
      ctx.fillStyle = '#09090b'; // zinc-950
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const width = canvas.width;
      const height = canvas.height;
      const centerY = height / 2;

      // Draw grid lines
      ctx.strokeStyle = '#27272a'; // zinc-800
      ctx.lineWidth = 1;
      
      // Vertical grid
      for (let x = 0; x < width; x += 50) {
        ctx.beginPath();
        ctx.moveTo(x, 0);
        ctx.lineTo(x, height);
        ctx.stroke();
      }

      // Horizontal grid
      for (let y = 0; y < height; y += 50) {
        ctx.beginPath();
        ctx.moveTo(0, y);
        ctx.lineTo(width, y);
        ctx.stroke();
      }

      // Draw waveform
      ctx.lineWidth = 3;
      ctx.strokeStyle = '#10b981'; // emerald-500
      ctx.beginPath();

      for (let x = 0; x < width; x++) {
        // Complex waveform synthesis
        const normalizedX = x / width;
        
        // Fundamental
        let y = Math.sin(normalizedX * Math.PI * 4 + time);
        
        // Harmonics
        y += Math.sin(normalizedX * Math.PI * 8 + time * 1.5) * 0.5;
        y += Math.sin(normalizedX * Math.PI * 16 + time * 2) * 0.25;
        
        // Modulation
        const mod = Math.sin(time * 0.5) * 0.5 + 0.5;
        y *= mod;

        // Map to screen coordinates
        const screenY = centerY + y * (height * 0.3);

        if (x === 0) {
          ctx.moveTo(x, screenY);
        } else {
          ctx.lineTo(x, screenY);
        }
      }
      
      ctx.stroke();

      // Draw "scan line"
      const scanX = (time * 100) % width;
      ctx.strokeStyle = 'rgba(255, 255, 255, 0.2)';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.moveTo(scanX, 0);
      ctx.lineTo(scanX, height);
      ctx.stroke();

      animationFrameId = requestAnimationFrame(draw);
    };

    draw();

    return () => {
      window.removeEventListener('resize', resize);
      cancelAnimationFrame(animationFrameId);
    };
  }, []);

  return <canvas ref={canvasRef} className="absolute inset-0 w-full h-full opacity-30" />;
}

export function Hero() {
  return (
    <div className="relative min-h-screen flex items-center justify-center overflow-hidden bg-zinc-950 pt-16">
      <WavetableVisualizer />
      
      <div className="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.8 }}
        >
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-500/10 border border-emerald-500/20 text-emerald-400 text-sm font-medium mb-8">
            <Terminal className="w-4 h-4" />
            <span>v0.1.0 Alpha Release</span>
          </div>
          
          <h1 className="text-5xl md:text-7xl font-bold tracking-tight text-white mb-6">
            Table<span className="text-emerald-500">Studio</span>
          </h1>
          
          <p className="text-xl md:text-2xl text-zinc-400 max-w-3xl mx-auto mb-10 leading-relaxed">
            The open-source wavetable workstation. <br className="hidden md:block" />
            Multi-frame editor and synthesizer plugin built on <span className="text-white font-mono">NIH-plug</span> and <span className="text-white font-mono">egui</span>.
          </p>

          <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
            <a
              href="#quickstart"
              className="w-full sm:w-auto inline-flex items-center justify-center gap-2 bg-white text-zinc-950 px-8 py-4 rounded-lg text-lg font-semibold hover:bg-zinc-200 transition-all transform hover:scale-105"
            >
              <Download className="w-5 h-5" />
              Get Started
            </a>
            <a
              href="https://github.com/sp80808/TableStudio"
              target="_blank"
              rel="noopener noreferrer"
              className="w-full sm:w-auto inline-flex items-center justify-center gap-2 bg-zinc-800 text-white px-8 py-4 rounded-lg text-lg font-semibold hover:bg-zinc-700 transition-all border border-white/10"
            >
              View Source <ArrowRight className="w-5 h-5" />
            </a>
          </div>

          <div className="mt-16 grid grid-cols-2 md:grid-cols-4 gap-8 text-zinc-500 text-sm font-mono">
            <div className="flex flex-col items-center gap-2">
              <span className="text-emerald-500 text-lg font-bold">Rust</span>
              <span>Powered Core</span>
            </div>
            <div className="flex flex-col items-center gap-2">
              <span className="text-emerald-500 text-lg font-bold">CLAP + VST3</span>
              <span>Plugin Formats</span>
            </div>
            <div className="flex flex-col items-center gap-2">
              <span className="text-emerald-500 text-lg font-bold">64 Frames</span>
              <span>Wavetable Grid</span>
            </div>
            <div className="flex flex-col items-center gap-2">
              <span className="text-emerald-500 text-lg font-bold">Open Source</span>
              <span>MIT License</span>
            </div>
          </div>
        </motion.div>
      </div>
      
      <div className="absolute bottom-0 left-0 right-0 h-32 bg-gradient-to-t from-zinc-950 to-transparent pointer-events-none" />
    </div>
  );
}
