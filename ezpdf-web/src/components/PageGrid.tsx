'use client';

/**
 * True-aspect page tiles built from the PDF's real page dimensions.
 *
 * Raycast's decorative vocabulary is app-icon tiles; ours is the user's own
 * pages, which is the same principle with our content substituted.
 */
import { useState } from 'react';

// 76px tiles are comfortable with a mouse; a finger needs more than the
// 44px minimum in both axes, and the aspect ratio makes height the easy one.
const BASE_WIDTH = 76;

export interface PageTile {
  pageNum: number;
  width: number;
  height: number;
  selected: boolean;
}

export function tilesFromDimensions(dimensions: [number, number][]): PageTile[] {
  return dimensions.map(([width, height], i) => ({
    pageNum: i + 1,
    width,
    height,
    selected: false,
  }));
}

export function PageGrid({
  tiles,
  onChange,
}: {
  tiles: PageTile[];
  onChange: (tiles: PageTile[]) => void;
}) {
  const [lastIndex, setLastIndex] = useState<number | null>(null);

  function toggle(index: number, shiftKey: boolean) {
    // Shift extends the run from the last plain click and turns it all on,
    // which is what "select pages 3 through 9" means in practice.
    if (shiftKey && lastIndex !== null) {
      const [from, to] = [Math.min(lastIndex, index), Math.max(lastIndex, index)];
      onChange(tiles.map((t, i) => (i >= from && i <= to ? { ...t, selected: true } : t)));
      return;
    }
    onChange(tiles.map((t, i) => (i === index ? { ...t, selected: !t.selected } : t)));
    setLastIndex(index);
  }

  const selectedCount = tiles.filter((t) => t.selected).length;

  return (
    <div>
      <div className="mb-2 flex items-center justify-between text-xs text-mute">
        <span>
          {selectedCount === 0
            ? 'Select pages — shift-click for a range'
            : `${selectedCount} of ${tiles.length} selected`}
        </span>
        {selectedCount > 0 && (
          <button
            type="button"
            className="flex min-h-11 items-center px-2 text-mute underline-offset-2 hover:text-ink hover:underline pointer-fine:min-h-0 pointer-fine:px-0"
            onClick={() => onChange(tiles.map((t) => ({ ...t, selected: false })))}
          >
            Clear
          </button>
        )}
      </div>

      <div className="flex max-h-64 flex-wrap gap-2 overflow-y-auto" role="group" aria-label="Pages">
        {tiles.map((tile, i) => (
          <button
            key={tile.pageNum}
            type="button"
            aria-pressed={tile.selected}
            aria-label={`Page ${tile.pageNum}`}
            onClick={(e) => toggle(i, e.shiftKey)}
            style={{
              width: BASE_WIDTH,
              height: tile.width > 0 ? (BASE_WIDTH * tile.height) / tile.width : BASE_WIDTH,
            }}
            className={`flex shrink-0 items-center justify-center rounded-md border text-sm font-semibold transition-colors ${
              tile.selected
                ? 'border-accent-blue bg-accent-blue-soft text-accent-blue'
                : 'border-hairline bg-card text-body hover:border-hairline-strong hover:text-ink'
            }`}
          >
            {tile.pageNum}
          </button>
        ))}
      </div>
    </div>
  );
}
