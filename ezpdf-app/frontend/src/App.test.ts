import { describe, it, expect, vi, beforeEach } from 'vitest';
import { render, screen, fireEvent } from '@testing-library/svelte';
import App from './App.svelte';

vi.mock('./lib/tauri', () => ({
  cmdMerge: vi.fn(),
  cmdSplitRange: vi.fn(),
  cmdSplitEach: vi.fn(),
  cmdRemove: vi.fn(),
  cmdRotate: vi.fn(),
  cmdReorder: vi.fn(),
}));

vi.mock('./lib/dnd', () => ({
  onFileDrop: vi.fn().mockResolvedValue(vi.fn()),
}));

import { cmdMerge } from './lib/tauri';
import { onFileDrop } from './lib/dnd';

const mockCmdMerge = vi.mocked(cmdMerge);
const mockOnFileDrop = vi.mocked(onFileDrop);

beforeEach(() => {
  vi.clearAllMocks();
  mockOnFileDrop.mockResolvedValue(vi.fn());
});

describe('App', () => {
  it('Run button is disabled when no files are loaded', () => {
    render(App);
    const runBtn = screen.getByRole('button', { name: /run/i });
    expect(runBtn).toBeDisabled();
  });

  it('Run button is enabled after files are added', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });

    render(App);

    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    const runBtn = screen.getByRole('button', { name: /run/i });
    await vi.waitFor(() => expect(runBtn).not.toBeDisabled());
  });

  it('clicking Run with merge op calls cmdMerge with inputs and output', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    mockCmdMerge.mockResolvedValue('Merged 1 files → /home/user/doc-merged.pdf');

    render(App);

    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    const runBtn = screen.getByRole('button', { name: /run/i });
    await fireEvent.click(runBtn);

    expect(mockCmdMerge).toHaveBeenCalledWith(
      ['/home/user/doc.pdf'],
      '/home/user/doc-merged.pdf'
    );
  });

  it('success result appears in status bar', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    mockCmdMerge.mockResolvedValue('Merged 1 files → /home/user/doc-merged.pdf');

    render(App);

    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    await fireEvent.click(screen.getByRole('button', { name: /run/i }));

    expect(
      await screen.findByText(/merged 1 files/i)
    ).toBeInTheDocument();
  });

  it('error result appears in status bar', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    mockCmdMerge.mockRejectedValue('encrypted PDF');

    render(App);

    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    await fireEvent.click(screen.getByRole('button', { name: /run/i }));

    expect(await screen.findByText(/encrypted pdf/i)).toBeInTheDocument();
  });

  it('default output path is same folder as first input with op suffix', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    mockCmdMerge.mockResolvedValue('ok');

    render(App);

    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/report.pdf']);

    await fireEvent.click(screen.getByRole('button', { name: /run/i }));

    expect(mockCmdMerge).toHaveBeenCalledWith(
      ['/home/user/report.pdf'],
      '/home/user/report-merged.pdf'
    );
  });
});
