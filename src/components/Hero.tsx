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
      ctx.fillStyle = '#fbf7f0';
      ctx.fillRect(0, 0, canvas.width, canvas.height);

      const width = canvas.width;
      const height = canvas.height;
      const centerY = height / 2;

      // Draw grid lines
      ctx.strokeStyle = '#e7dfd3';
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
      ctx.strokeStyle = '#0f766e';
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
      ctx.strokeStyle = 'rgba(31, 27, 22, 0.12)';
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

  return <canvas ref={canvasRef} className="absolute inset-0 w-full h-full opacity-60" />;
}

export function Hero() {
  return (
    <div className="relative min-h-screen pt-24 overflow-hidden">
      <div className="absolute -top-32 right-[-10%] h-[32rem] w-[32rem] rounded-full bg-emerald-200/40 blur-3xl" />
      <div className="absolute -bottom-24 left-[-10%] h-[26rem] w-[26rem] rounded-full bg-sky-200/40 blur-3xl" />

      <div className="relative z-10 max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid lg:grid-cols-[1.15fr_0.85fr] gap-12 items-center">
          <motion.div
            initial={{ opacity: 0, y: 18 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.8 }}
          >
            <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-emerald-100 border border-emerald-200 text-emerald-700 text-xs font-semibold tracking-wide mb-6">
              <Terminal className="w-4 h-4" />
              <span>v0.1.0 alpha build</span>
            </div>

            <h1 className="text-5xl md:text-6xl font-bold tracking-tight text-[#1f1b16] mb-6">
              TableStudio, a wavetable workstation built for deep sound design.
            </h1>

            <p className="text-lg md:text-xl text-[#5b5346] max-w-2xl mb-8 leading-relaxed">
              Shape multi-frame tables with a drag-to-edit canvas, real-time harmonics view, and a
              non-destructive bake pipeline. Built on <span className="font-mono text-[#1f1b16]">NIH-plug</span>
              and <span className="font-mono text-[#1f1b16]">egui</span>.
            </p>

            <div className="flex flex-col sm:flex-row items-center justify-start gap-4">
              <a
                href="https://github.com/sp80808/TableStudio/releases"
                target="_blank"
                rel="noopener noreferrer"
                className="w-full sm:w-auto inline-flex items-center justify-center gap-2 bg-[#1f1b16] text-[#fdf8f1] px-7 py-3 rounded-full text-base font-semibold hover:bg-[#2d2722] transition-all"
              >
                <Download className="w-5 h-5" />
                Download (Releases)
              </a>
              <a
                href="https://github.com/sp80808/TableStudio"
                target="_blank"
                rel="noopener noreferrer"
                className="w-full sm:w-auto inline-flex items-center justify-center gap-2 border border-black/10 bg-white/70 text-[#1f1b16] px-7 py-3 rounded-full text-base font-semibold hover:bg-white transition-all"
              >
                View Source <ArrowRight className="w-5 h-5" />
              </a>
            </div>

            <div className="mt-10 flex flex-wrap gap-4 text-xs font-semibold tracking-wide text-[#7a6f61]">
              <span className="px-3 py-1 rounded-full bg-white/70 border border-black/5">Rust core</span>
              <span className="px-3 py-1 rounded-full bg-white/70 border border-black/5">CLAP + VST3</span>
              <span className="px-3 py-1 rounded-full bg-white/70 border border-black/5">64 frame grid</span>
              <span className="px-3 py-1 rounded-full bg-white/70 border border-black/5">MIT license</span>
            </div>
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 24 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.9, delay: 0.1 }}
            className="relative"
          >
            <div className="relative rounded-3xl border border-black/10 bg-white/70 shadow-xl shadow-black/5 overflow-hidden">
              <div className="absolute inset-0">
                <WavetableVisualizer />
              </div>
              <div className="relative p-6 md:p-8">
                <div className="flex items-center justify-between text-xs font-semibold text-[#7a6f61]">
                  <span>Wavetable scan</span>
                  <span>Live preview</span>
                </div>
                <div className="mt-48 md:mt-56 grid grid-cols-2 gap-4">
                  <div className="rounded-2xl bg-white/80 border border-black/5 p-4">
                    <p className="text-sm text-[#5b5346]">FM stack</p>
                    <p className="text-lg font-semibold text-[#1f1b16]">Carrier + Mod</p>
                  </div>
                  <div className="rounded-2xl bg-white/80 border border-black/5 p-4">
                    <p className="text-sm text-[#5b5346]">Preview mode</p>
                    <p className="text-lg font-semibold text-[#1f1b16]">Edit-Drone</p>
                  </div>
                </div>
              </div>
            </div>
          </motion.div>
        </div>
      </div>
    </div>
  );
}
