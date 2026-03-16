import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import PageGrid from './PageGrid.svelte';

interface PageTile {
  pageNum: number;
  width: number;
  height: number;
  selected: boolean;
}

function makeTiles(count: number): PageTile[] {
  return Array.from({ length: count }, (_, i) => ({
    pageNum: i + 1,
    width: 612,
    height: 792,
    selected: false,
  }));
}

describe('PageGrid', () => {
  it('renders correct number of tiles', () => {
    const tiles = makeTiles(3);
    render(PageGrid, { tiles, mode: 'select', ontiles: vi.fn() });
    const tileDivs = screen.getAllByRole('button');
    expect(tileDivs).toHaveLength(3);
  });

  it('each tile shows its page number', () => {
    const tiles = makeTiles(3);
    render(PageGrid, { tiles, mode: 'select', ontiles: vi.fn() });
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
  });

  it('tiles have proportional aspect ratio via inline style', () => {
    const tiles = [{ pageNum: 1, width: 200, height: 100, selected: false }];
    render(PageGrid, { tiles, mode: 'select', ontiles: vi.fn() });
    const btn = screen.getByRole('button');
    // height = 88 * (100/200) = 44px
    expect(btn.style.height).toBe('44px');
  });

  it('click toggles selected and calls ontiles with updated array', async () => {
    const tiles = makeTiles(3);
    const ontiles = vi.fn();
    render(PageGrid, { tiles, mode: 'select', ontiles });

    const buttons = screen.getAllByRole('button');
    await fireEvent.click(buttons[1]); // click page 2

    expect(ontiles).toHaveBeenCalledOnce();
    const updated: PageTile[] = ontiles.mock.calls[0][0];
    expect(updated[1].selected).toBe(true);
    expect(updated[0].selected).toBe(false);
    expect(updated[2].selected).toBe(false);
  });

  it('shift-click selects a range and calls ontiles', async () => {
    const tiles = makeTiles(5);
    const ontiles = vi.fn();
    render(PageGrid, { tiles, mode: 'select', ontiles });

    const buttons = screen.getAllByRole('button');
    // plain click page 1
    await fireEvent.click(buttons[0]);
    ontiles.mockClear();

    // shift-click page 4 → selects pages 1-4
    await fireEvent.click(buttons[3], { shiftKey: true });
    expect(ontiles).toHaveBeenCalledOnce();
    const updated: PageTile[] = ontiles.mock.calls[0][0];
    expect(updated[0].selected).toBe(true);
    expect(updated[1].selected).toBe(true);
    expect(updated[2].selected).toBe(true);
    expect(updated[3].selected).toBe(true);
    expect(updated[4].selected).toBe(false);
  });

  it('selected tile has data-selected attribute', async () => {
    const tiles = [{ pageNum: 1, width: 612, height: 792, selected: true }];
    render(PageGrid, { tiles, mode: 'select', ontiles: vi.fn() });
    const btn = screen.getByRole('button');
    expect(btn).toHaveAttribute('data-selected', 'true');
  });

  it('pointer-drag in reorder mode reorders tiles and calls ontiles', async () => {
    const tiles = makeTiles(3);
    const ontiles = vi.fn();
    const { container } = render(PageGrid, { tiles, mode: 'reorder', ontiles });

    const buttons = screen.getAllByRole('button');
    // pointerdown on tile 0, move on grid, pointerup on tile 2
    await fireEvent.pointerDown(buttons[0]);
    await fireEvent.pointerMove(container.querySelector('.page-grid')!);
    await fireEvent.pointerUp(buttons[2]);

    expect(ontiles).toHaveBeenCalledOnce();
    const updated: PageTile[] = ontiles.mock.calls[0][0];
    // page 1 moved to end: [2, 3, 1]
    expect(updated.map((t) => t.pageNum)).toEqual([2, 3, 1]);
  });

  it('does not call ontiles when released on same tile in reorder mode', async () => {
    const tiles = makeTiles(3);
    const ontiles = vi.fn();
    const { container } = render(PageGrid, { tiles, mode: 'reorder', ontiles });

    const buttons = screen.getAllByRole('button');
    await fireEvent.pointerDown(buttons[1]);
    await fireEvent.pointerMove(container.querySelector('.page-grid')!);
    await fireEvent.pointerUp(buttons[1]);

    expect(ontiles).not.toHaveBeenCalled();
  });
});
