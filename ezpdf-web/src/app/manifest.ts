import type { MetadataRoute } from 'next';

export default function manifest(): MetadataRoute.Manifest {
  return {
    name: 'ezpdf — every PDF tool, without the upload',
    short_name: 'ezpdf',
    description:
      'Merge, split, rotate and convert PDFs to Markdown entirely in your browser.',
    start_url: '/app',
    display: 'standalone',
    background_color: '#07080a',
    theme_color: '#07080a',
    icons: [
      { src: '/icon.svg', sizes: 'any', type: 'image/svg+xml' },
      { src: '/apple-icon.png', sizes: '180x180', type: 'image/png' },
    ],
  };
}
