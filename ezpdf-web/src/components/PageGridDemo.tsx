'use client';

import { useState } from 'react';

import { PageGrid, type PageTile } from './PageGrid';

const A4: [number, number] = [612, 792];

export function PageGridDemo() {
  const [tiles, setTiles] = useState<PageTile[]>(
    Array.from({ length: 6 }, (_, i) => ({
      pageNum: i + 1,
      width: A4[0],
      height: A4[1],
      selected: i === 1 || i === 2,
    })),
  );
  return <PageGrid tiles={tiles} onChange={setTiles} />;
}
