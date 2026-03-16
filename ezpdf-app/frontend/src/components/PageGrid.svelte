<script lang="ts">
  interface PageTile {
    pageNum: number;
    width: number;
    height: number;
    selected: boolean;
  }

  let {
    tiles,
    mode,
    ontiles,
  }: {
    tiles: PageTile[];
    mode: 'reorder' | 'select';
    ontiles: (updated: PageTile[]) => void;
  } = $props();

  const BASE_WIDTH = 88;

  let draggingIndex: number | null = $state(null);
  let isDragging = $state(false);
  let lastSelectedIndex: number | null = $state(null);

  function tileHeight(tile: PageTile): number {
    if (tile.width <= 0) return BASE_WIDTH;
    return BASE_WIDTH * (tile.height / tile.width);
  }

  function handlePointerDown(index: number) {
    if (mode !== 'reorder') return;
    draggingIndex = index;
    isDragging = false;
  }

  function handlePointerMove() {
    if (mode === 'reorder' && draggingIndex !== null) isDragging = true;
  }

  function handlePointerUp(index: number) {
    if (mode === 'reorder' && isDragging && draggingIndex !== null && draggingIndex !== index) {
      const updated = [...tiles];
      const [moved] = updated.splice(draggingIndex, 1);
      updated.splice(index, 0, moved);
      ontiles(updated);
    }
    draggingIndex = null;
    isDragging = false;
  }

  function handlePointerCancel() {
    draggingIndex = null;
    isDragging = false;
  }

  function handleClick(index: number, event: MouseEvent) {
    if (mode !== 'select') return;
    // Suppress click if this was actually a pointer-drag gesture
    if (isDragging) return;

    if (event.shiftKey && lastSelectedIndex !== null) {
      const lo = Math.min(lastSelectedIndex, index);
      const hi = Math.max(lastSelectedIndex, index);
      const updated = tiles.map((t, i) => ({
        ...t,
        selected: i >= lo && i <= hi ? true : t.selected,
      }));
      ontiles(updated);
    } else {
      const updated = tiles.map((t, i) => ({
        ...t,
        selected: i === index ? !t.selected : t.selected,
      }));
      ontiles(updated);
      lastSelectedIndex = index;
    }
  }
</script>

<div class="page-grid" onpointermove={handlePointerMove}>
  {#each tiles as tile, i}
    <button
      class="tile"
      class:selected={tile.selected}
      class:dragging={draggingIndex === i && isDragging}
      data-selected={tile.selected ? 'true' : undefined}
      style="width: {BASE_WIDTH}px; height: {tileHeight(tile)}px;"
      data-pos={mode === 'reorder' ? String(i + 1) : undefined}
      onpointerdown={() => handlePointerDown(i)}
      onpointerup={() => handlePointerUp(i)}
      onpointercancel={handlePointerCancel}
      onclick={(e) => handleClick(i, e)}
    >
      {tile.pageNum}
    </button>
  {/each}
</div>

<style>
  .page-grid {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
    max-height: 300px;
    overflow-y: auto;
  }

  .tile {
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    background: #f3f4f6;
    border: 2px solid #d1d5db;
    border-radius: 4px;
    font-size: 0.875rem;
    font-weight: 600;
    color: #374151;
    cursor: pointer;
    user-select: none;
    flex-shrink: 0;
  }

  .tile.selected {
    border-color: #3b82f6;
    background: #eff6ff;
    color: #1d4ed8;
  }

  .tile.dragging {
    opacity: 0.4;
  }

  .tile[data-pos]::before {
    content: attr(data-pos);
    position: absolute;
    top: 4px;
    left: 0;
    right: 0;
    text-align: center;
    font-size: 0.6rem;
    font-weight: 400;
    color: #9ca3af;
    line-height: 1;
  }

  .tile:hover:not(.selected) {
    border-color: #6b7280;
    background: #e5e7eb;
    color: #111827;
  }
</style>
