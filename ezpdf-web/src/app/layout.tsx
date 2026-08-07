import type { Metadata, Viewport } from 'next';
import { Inter } from 'next/font/google';

import { Nav } from '@/components/Nav';
import './globals.css';

// ss03 is part of the brand identity — the alternate `g`.
const inter = Inter({ subsets: ['latin'], variable: '--font-inter' });

export const viewport: Viewport = {
  width: 'device-width',
  initialScale: 1,
  // Matches the canvas so the iOS status bar and browser chrome blend in.
  themeColor: '#07080a',
  colorScheme: 'dark',
};

export const metadata: Metadata = {
  title: 'ezpdf — every PDF tool, without the upload',
  description:
    'Merge, split, rotate and convert PDFs to Markdown entirely in your browser. Nothing is uploaded unless you choose to save it.',
  // Controls the label and status bar when saved to an iOS home screen.
  appleWebApp: { capable: true, title: 'ezpdf', statusBarStyle: 'black-translucent' },
  manifest: '/manifest.webmanifest',
};

export default function RootLayout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en" className={inter.variable}>
      <body>
        <Nav />
        {children}
      </body>
    </html>
  );
}
