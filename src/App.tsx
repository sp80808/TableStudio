/**
 * @license
 * SPDX-License-Identifier: Apache-2.0
 */

import { Navbar } from './components/Navbar';
import { Hero } from './components/Hero';
import { Downloads } from './components/Downloads';
import { Features } from './components/Features';
import { Technical } from './components/Technical';
import { Sources } from './components/Sources';
import { Support } from './components/Support';
import { Footer } from './components/Footer';

export default function App() {
  return (
    <div className="min-h-screen text-[#1f1b16] selection:bg-emerald-200/70">
      <Navbar />
      <main>
        <Hero />
        <Downloads />
        <Features />
        <Technical />
        <Sources />
        <Support />
      </main>
      <Footer />
    </div>
  );
}
