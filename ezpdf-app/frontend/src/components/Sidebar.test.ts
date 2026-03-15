import { describe, it, expect, vi } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import Sidebar from './Sidebar.svelte';

describe('Sidebar', () => {
  it('renders 5 operation buttons: Merge, Split, Remove, Rotate, Reorder', () => {
    render(Sidebar);
    expect(screen.getByRole('button', { name: /merge/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /split/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /remove/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /rotate/i })).toBeInTheDocument();
    expect(screen.getByRole('button', { name: /reorder/i })).toBeInTheDocument();
  });

  it('calls onopSelected callback with operation name when button is clicked', async () => {
    const received: string[] = [];
    render(Sidebar, { onopSelected: (op: string) => received.push(op) });

    await fireEvent.click(screen.getByRole('button', { name: /merge/i }));
    expect(received).toEqual(['merge']);

    await fireEvent.click(screen.getByRole('button', { name: /split/i }));
    expect(received).toEqual(['merge', 'split']);
  });

  it('selected op has active CSS class', async () => {
    render(Sidebar, { selectedOp: 'merge' });
    const mergeBtn = screen.getByRole('button', { name: /merge/i });
    expect(mergeBtn).toHaveClass('active');
    const splitBtn = screen.getByRole('button', { name: /split/i });
    expect(splitBtn).not.toHaveClass('active');
  });
});
