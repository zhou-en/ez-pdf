import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import FileList from './FileList.svelte';

const files = ['/home/user/a.pdf', '/home/user/b.pdf', '/home/user/c.pdf'];
const pageCounts: Record<string, number> = {
  '/home/user/a.pdf': 3,
  '/home/user/b.pdf': 1,
  '/home/user/c.pdf': 10,
};

describe('FileList', () => {
  it('renders basenames for each file', () => {
    render(FileList, { files, pageCounts: {}, onremove: vi.fn(), onreorder: vi.fn() });
    expect(screen.getByText('a.pdf')).toBeInTheDocument();
    expect(screen.getByText('b.pdf')).toBeInTheDocument();
    expect(screen.getByText('c.pdf')).toBeInTheDocument();
  });

  it('shows page count next to each file', () => {
    render(FileList, { files, pageCounts, onremove: vi.fn(), onreorder: vi.fn() });
    expect(screen.getByText('3 pages')).toBeInTheDocument();
    expect(screen.getByText('1 page')).toBeInTheDocument();
    expect(screen.getByText('10 pages')).toBeInTheDocument();
  });

  it('calls onremove with correct index when remove button clicked', async () => {
    const onremove = vi.fn();
    render(FileList, { files, pageCounts: {}, onremove, onreorder: vi.fn() });
    await fireEvent.click(screen.getByRole('button', { name: /remove b\.pdf/i }));
    expect(onremove).toHaveBeenCalledWith(1);
  });

  it('calls onreorder(from, to) when an item is pointer-dragged onto another', async () => {
    const onreorder = vi.fn();
    const { container } = render(FileList, { files, pageCounts: {}, onremove: vi.fn(), onreorder });

    const items = screen.getAllByRole('listitem');

    // pointerdown on item 0, move on container, pointerup on item 2
    await fireEvent.pointerDown(items[0]);
    await fireEvent.pointerMove(container.querySelector('.file-list')!);
    await fireEvent.pointerUp(items[2]);

    expect(onreorder).toHaveBeenCalledWith(0, 2);
  });

  it('does not call onreorder when released on the same item', async () => {
    const onreorder = vi.fn();
    const { container } = render(FileList, { files, pageCounts: {}, onremove: vi.fn(), onreorder });

    const items = screen.getAllByRole('listitem');
    await fireEvent.pointerDown(items[1]);
    await fireEvent.pointerMove(container.querySelector('.file-list')!);
    await fireEvent.pointerUp(items[1]);

    expect(onreorder).not.toHaveBeenCalled();
  });
});
