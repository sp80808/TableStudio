import { Check, Copy, Heart } from 'lucide-react';
import { useState } from 'react';

const PAYPAL_LINK = 'https://paypal.me/hsp8m8';
const DONATION_LABEL = 'Support';

const cryptoAddresses = [
  { ticker: 'BTC', address: '3MCm29V2AfHdGKW5bAu8aAJjVQbHu4aeYC' },
  { ticker: 'ETH', address: '0xC32eA3793b2F0183bb6fB598A3Fa98CeB6B24DDb' },
];

function CopyRow({ ticker, address }: { ticker: string; address: string }) {
  const [copied, setCopied] = useState(false);

  const handleCopy = async () => {
    await navigator.clipboard.writeText(address);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="flex flex-col gap-2 rounded-xl border border-black/10 bg-white/70 p-4">
      <div className="flex items-center justify-between">
        <span className="text-sm font-semibold text-[#1f1b16]">{ticker}</span>
        <button
          type="button"
          onClick={handleCopy}
          className="inline-flex items-center gap-2 text-xs font-semibold text-emerald-700 hover:text-emerald-900"
        >
          {copied ? <Check className="w-4 h-4" /> : <Copy className="w-4 h-4" />}
          {copied ? 'Copied' : 'Copy'}
        </button>
      </div>
      <code className="text-xs text-[#5b5346] break-all">{address}</code>
    </div>
  );
}

export function Support() {
  return (
    <section id="support" className="py-24">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="grid lg:grid-cols-[1.05fr_0.95fr] gap-12 items-start">
          <div>
            <p className="text-xs uppercase tracking-[0.3em] text-emerald-600 font-semibold">
              {DONATION_LABEL}
            </p>
            <h2 className="text-3xl md:text-4xl font-bold text-[#1f1b16] mt-3">
              Keep TableStudio moving forward
            </h2>
            <p className="text-lg text-[#5b5346] mt-4 max-w-xl">
              TableStudio is built in the open. If the project helps your music workflow, consider
              supporting continued development.
            </p>
            <div className="mt-8">
              <a
                href={PAYPAL_LINK}
                target="_blank"
                rel="noopener noreferrer"
                className="inline-flex items-center gap-2 px-6 py-3 rounded-full bg-emerald-600 text-white font-semibold shadow-lg shadow-emerald-500/20 hover:bg-emerald-700 transition"
              >
                <Heart className="w-5 h-5" />
                Donate with PayPal
              </a>
              <p className="text-sm text-[#7a6f61] mt-3">Secure checkout via PayPal.</p>
            </div>
          </div>

          <div className="rounded-3xl border border-black/10 bg-[#f1ebe1] p-8 shadow-sm">
            <h3 className="text-lg font-semibold text-[#1f1b16] mb-4">Crypto</h3>
            <p className="text-sm text-[#5b5346] mb-6">
              Send crypto directly. Please double-check the address before sending.
            </p>
            <div className="grid gap-4">
              {cryptoAddresses.map((entry) => (
                <CopyRow key={entry.ticker} ticker={entry.ticker} address={entry.address} />
              ))}
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}
