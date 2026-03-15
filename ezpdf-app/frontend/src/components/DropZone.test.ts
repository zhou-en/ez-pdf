import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen } from '@testing-library/svelte';
import DropZone from './DropZone.svelte';

vi.mock('../lib/dnd', () => ({
  onFileDrop: vi.fn(),
}));

import { onFileDrop } from '../lib/dnd';

const mockOnFileDrop = vi.mocked(onFileDrop);

beforeEach(() => {
  mockOnFileDrop.mockReset();
  mockOnFileDrop.mockResolvedValue(vi.fn()); // returns unlisten fn
});

describe('DropZone', () => {
  it('renders drop zone with instructions text', () => {
    render(DropZone);
    expect(screen.getByText(/drop pdf files here/i)).toBeInTheDocument();
  });

  it('emits filesAdded event when onFileDrop callback fires with pdf paths', async () => {
    let capturedHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      capturedHandler = handler;
      return vi.fn();
    });

    const { component } = render(DropZone);
    const received: string[][] = [];
    component.$on('filesAdded', (e: CustomEvent<string[]>) => {
      received.push(e.detail);
    });

    // Wait for onMount to call onFileDrop
    await vi.waitFor(() => expect(capturedHandler).toBeDefined());
    capturedHandler!(['/home/user/doc.pdf', '/home/user/report.pdf']);

    expect(received).toHaveLength(1);
    expect(received[0]).toEqual(['/home/user/doc.pdf', '/home/user/report.pdf']);
  });

  it('filters out non-PDF paths from the drop event', async () => {
    let capturedHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      capturedHandler = handler;
      return vi.fn();
    });

    const { component } = render(DropZone);
    const received: string[][] = [];
    component.$on('filesAdded', (e: CustomEvent<string[]>) => {
      received.push(e.detail);
    });

    await vi.waitFor(() => expect(capturedHandler).toBeDefined());
    capturedHandler!(['/home/user/doc.pdf', '/home/user/image.png', '/home/user/notes.txt']);

    expect(received).toHaveLength(1);
    expect(received[0]).toEqual(['/home/user/doc.pdf']);
  });

  it('shows dropped file basenames in the component', async () => {
    let capturedHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      capturedHandler = handler;
      return vi.fn();
    });

    render(DropZone);

    await vi.waitFor(() => expect(capturedHandler).toBeDefined());
    capturedHandler!(['/home/user/report.pdf']);

    expect(await screen.findByText('report.pdf')).toBeInTheDocument();
  });
});
