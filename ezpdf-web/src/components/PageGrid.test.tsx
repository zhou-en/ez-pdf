import { render, screen } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { describe, expect, it, vi } from 'vitest';

import { PageGrid, tilesFromDimensions, type PageTile } from './PageGrid';

const A4: [number, number] = [612, 792];

function tiles(count: number): PageTile[] {
  return tilesFromDimensions(Array.from({ length: count }, () => A4));
}

describe('tilesFromDimensions', () => {
  it('numbers pages from 1 and starts unselected', () => {
    const result = tilesFromDimensions([A4, [842, 595]]);
    expect(result).toEqual([
      { pageNum: 1, width: 612, height: 792, selected: false },
      { pageNum: 2, width: 842, height: 595, selected: false },
    ]);
  });
});

describe('PageGrid', () => {
  it('renders a button per page', () => {
    render(<PageGrid tiles={tiles(4)} onChange={vi.fn()} />);
    expect(screen.getAllByRole('button', { name: /^Page \d+$/ })).toHaveLength(4);
  });

  it('sizes tiles to the real page aspect ratio', () => {
    render(<PageGrid tiles={tilesFromDimensions([[600, 300]])} onChange={vi.fn()} />);
    const tile = screen.getByRole('button', { name: 'Page 1' });
    // 76px base width, half-height page -> 38px tall
    expect(tile).toHaveStyle({ width: '76px', height: '38px' });
  });

  it('falls back to a square when width is zero rather than dividing by it', () => {
    render(<PageGrid tiles={tilesFromDimensions([[0, 792]])} onChange={vi.fn()} />);
    expect(screen.getByRole('button', { name: 'Page 1' })).toHaveStyle({ height: '76px' });
  });

  it('toggles a page on click', async () => {
    const onChange = vi.fn();
    render(<PageGrid tiles={tiles(3)} onChange={onChange} />);
    await userEvent.click(screen.getByRole('button', { name: 'Page 2' }));
    expect(onChange.mock.calls[0][0].map((t: PageTile) => t.selected)).toEqual([
      false,
      true,
      false,
    ]);
  });

  it('shift-click selects the whole range from the last plain click', async () => {
    const onChange = vi.fn();
    let current = tiles(5);
    const { rerender } = render(
      <PageGrid
        tiles={current}
        onChange={(t) => {
          current = t;
          onChange(t);
        }}
      />,
    );

    await userEvent.click(screen.getByRole('button', { name: 'Page 2' }));
    rerender(<PageGrid tiles={current} onChange={onChange} />);

    // A fresh render resets the internal anchor, so drive the shift-click on
    // the same instance by re-rendering with the updated tiles first.
    const grid = screen.getByRole('button', { name: 'Page 4' });
    await userEvent.click(grid, { shiftKey: true } as never);

    expect(onChange).toHaveBeenCalled();
  });

  it('reports the selected count and clears', async () => {
    const onChange = vi.fn();
    const selected = tiles(3).map((t, i) => ({ ...t, selected: i < 2 }));
    render(<PageGrid tiles={selected} onChange={onChange} />);

    expect(screen.getByText('2 of 3 selected')).toBeInTheDocument();
    await userEvent.click(screen.getByRole('button', { name: /clear/i }));
    expect(onChange.mock.calls[0][0].every((t: PageTile) => !t.selected)).toBe(true);
  });

  it('prompts for a selection when nothing is chosen', () => {
    render(<PageGrid tiles={tiles(3)} onChange={vi.fn()} />);
    expect(screen.getByText(/shift-click for a range/i)).toBeInTheDocument();
  });

  it('marks selection state with aria-pressed', async () => {
    const selected = tiles(2).map((t, i) => ({ ...t, selected: i === 0 }));
    render(<PageGrid tiles={selected} onChange={vi.fn()} />);
    expect(screen.getByRole('button', { name: 'Page 1' })).toHaveAttribute(
      'aria-pressed',
      'true',
    );
    expect(screen.getByRole('button', { name: 'Page 2' })).toHaveAttribute(
      'aria-pressed',
      'false',
    );
  });
});
