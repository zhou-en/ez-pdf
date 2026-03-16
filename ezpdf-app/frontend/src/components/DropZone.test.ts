import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import DropZone from './DropZone.svelte';

vi.mock('../lib/dnd', () => ({
  onFileDrop: vi.fn(),
}));

vi.mock('../lib/dialog', () => ({
  openPdfFiles: vi.fn(),
}));

import { onFileDrop } from '../lib/dnd';
import { openPdfFiles } from '../lib/dialog';

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

  it('calls onfilesAdded callback when onFileDrop fires with pdf paths', async () => {
    let capturedHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      capturedHandler = handler;
      return vi.fn();
    });

    const received: string[][] = [];
    render(DropZone, { onfilesAdded: (paths: string[]) => received.push(paths) });

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

    const received: string[][] = [];
    render(DropZone, { onfilesAdded: (paths: string[]) => received.push(paths) });

    await vi.waitFor(() => expect(capturedHandler).toBeDefined());
    capturedHandler!(['/home/user/doc.pdf', '/home/user/image.png', '/home/user/notes.txt']);

    expect(received).toHaveLength(1);
    expect(received[0]).toEqual(['/home/user/doc.pdf']);
  });

  it('does not render a file list — display is owned by the parent', () => {
    render(DropZone);
    expect(document.querySelector('ul')).toBeNull();
  });

  it('clicking the drop zone opens the file dialog and emits selected pdfs', async () => {
    const mockOpen = vi.mocked(openPdfFiles);
    mockOpen.mockResolvedValue(['/home/user/a.pdf', '/home/user/b.pdf']);

    const received: string[][] = [];
    render(DropZone, { onfilesAdded: (paths: string[]) => received.push(paths) });

    await fireEvent.click(screen.getByRole('button', { name: /browse/i }));

    await vi.waitFor(() => expect(received).toHaveLength(1));
    expect(received[0]).toEqual(['/home/user/a.pdf', '/home/user/b.pdf']);
  });

  it('clicking browse does nothing when dialog is cancelled', async () => {
    const mockOpen = vi.mocked(openPdfFiles);
    mockOpen.mockResolvedValue(null);

    const received: string[][] = [];
    render(DropZone, { onfilesAdded: (paths: string[]) => received.push(paths) });

    await fireEvent.click(screen.getByRole('button', { name: /browse/i }));
    await vi.waitFor(() => expect(mockOpen).toHaveBeenCalled());
    expect(received).toHaveLength(0);
  });
});
