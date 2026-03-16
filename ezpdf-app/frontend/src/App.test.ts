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
  cmdPageCount: vi.fn().mockResolvedValue(5),
}));

vi.mock('./lib/dnd', () => ({
  onFileDrop: vi.fn().mockResolvedValue(vi.fn()),
}));

vi.mock('./lib/dialog', () => ({
  openPdfFiles: vi.fn(),
  saveOutputPath: vi.fn(),
  pickOutputDir: vi.fn(),
}));

import { cmdMerge } from './lib/tauri';
import { onFileDrop } from './lib/dnd';
import { saveOutputPath } from './lib/dialog';

const mockCmdMerge = vi.mocked(cmdMerge);
const mockOnFileDrop = vi.mocked(onFileDrop);
const mockSaveOutputPath = vi.mocked(saveOutputPath);

beforeEach(() => {
  vi.clearAllMocks();
  mockOnFileDrop.mockResolvedValue(vi.fn());
  mockSaveOutputPath.mockResolvedValue(null);
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

  it('files added in merge are not shown when switching to split', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });

    render(App);

    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    await vi.waitFor(() => expect(screen.getByText('doc.pdf')).toBeInTheDocument());

    await fireEvent.click(screen.getByRole('button', { name: /split/i }));

    expect(screen.queryByText('doc.pdf')).not.toBeInTheDocument();
  });

  it('remove button on a file removes it from the list', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });

    render(App);

    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    await vi.waitFor(() => expect(screen.getByText('doc.pdf')).toBeInTheDocument());

    await fireEvent.click(screen.getByRole('button', { name: /remove doc\.pdf/i }));

    expect(screen.queryByText('doc.pdf')).not.toBeInTheDocument();
  });

  it('shows page count next to each file after it is added', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    const { cmdPageCount } = await import('./lib/tauri');
    vi.mocked(cmdPageCount).mockResolvedValue(3);

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    expect(await screen.findByText(/3 pages/i)).toBeInTheDocument();
  });

  it('shows "1 page" (singular) for a single-page file', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    const { cmdPageCount } = await import('./lib/tauri');
    vi.mocked(cmdPageCount).mockResolvedValue(1);

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/single.pdf']);

    expect(await screen.findByText(/1 page\b/i)).toBeInTheDocument();
    expect(screen.queryByText(/1 pages/i)).not.toBeInTheDocument();
  });

  it('burst mode output path is a directory (no .pdf), range mode is a .pdf file', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    const mocks = await import('./lib/tauri');
    vi.mocked(mocks.cmdSplitRange).mockResolvedValue('ok');
    vi.mocked(mocks.cmdSplitEach).mockResolvedValue('ok');

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    await fireEvent.click(screen.getByRole('button', { name: /^split$/i }));
    dropHandler!(['/home/user/doc.pdf']);
    await vi.waitFor(() => expect(screen.getByRole('button', { name: /run split/i })).not.toBeDisabled());

    // Switch to burst — output should NOT end in .pdf
    await fireEvent.change(screen.getByLabelText(/burst all pages/i));
    await fireEvent.click(screen.getByRole('button', { name: /run split/i }));
    await vi.waitFor(() => expect(vi.mocked(mocks.cmdSplitEach)).toHaveBeenCalled());
    const burstCall = vi.mocked(mocks.cmdSplitEach).mock.calls[0];
    expect(burstCall[1]).not.toMatch(/\.pdf$/i);

    // Switch back to range — output should end in .pdf
    await fireEvent.change(screen.getByLabelText(/extract range/i));
    const rangeInput = screen.getByRole('textbox', { name: /range/i });
    await fireEvent.input(rangeInput, { target: { value: '1-2' } });
    await fireEvent.click(screen.getByRole('button', { name: /run split/i }));
    await vi.waitFor(() => expect(vi.mocked(mocks.cmdSplitRange)).toHaveBeenCalled());
    const rangeCall = vi.mocked(mocks.cmdSplitRange).mock.calls[0];
    expect(rangeCall[2]).toMatch(/\.pdf$/i);
    // Crucially, range output path must differ from burst output path
    expect(rangeCall[2]).not.toBe(burstCall[1]);
  });

  it('typed split range reaches the backend', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    const mockSplitRange = vi.mocked((await import('./lib/tauri')).cmdSplitRange);
    mockSplitRange.mockResolvedValue('Split → /home/user/doc-split.pdf');

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    await fireEvent.click(screen.getByRole('button', { name: /^split$/i }));
    dropHandler!(['/home/user/doc.pdf']);

    await vi.waitFor(() => expect(screen.getByRole('button', { name: /run split/i })).not.toBeDisabled());

    const rangeInput = screen.getByPlaceholderText('e.g. 1-3');
    await fireEvent.input(rangeInput, { target: { value: '1-2' } });

    await fireEvent.click(screen.getByRole('button', { name: /run split/i }));

    await vi.waitFor(() =>
      expect(mockSplitRange).toHaveBeenCalledWith('/home/user/doc.pdf', '1-2', expect.any(String))
    );
  });

  it('shows validation error when split range mode is selected but range is empty', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());

    // Switch to Split first, then drop a file so it belongs to Split's file list
    await fireEvent.click(screen.getByRole('button', { name: /^split$/i }));
    dropHandler!(['/home/user/doc.pdf']);

    await vi.waitFor(() => expect(screen.getByRole('button', { name: /run split/i })).not.toBeDisabled());
    await fireEvent.click(screen.getByRole('button', { name: /run split/i }));

    expect(await screen.findByText(/enter a page range/i)).toBeInTheDocument();
  });

  it('status message clears when switching operations', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    mockCmdMerge.mockResolvedValue('Merged 1 files → /home/user/doc-merged.pdf');

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);
    await fireEvent.click(screen.getByRole('button', { name: /run merge/i }));
    expect(await screen.findByText(/merged 1 files/i)).toBeInTheDocument();

    await fireEvent.click(screen.getByRole('button', { name: /^split$/i }));
    expect(screen.queryByText(/merged 1 files/i)).not.toBeInTheDocument();
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

  it('Save As button is visible when a file is loaded', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    await vi.waitFor(() => expect(screen.getByRole('button', { name: /save as/i })).toBeInTheDocument());
  });

  it('Save As path overrides the default output when Run is clicked', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    mockCmdMerge.mockResolvedValue('ok');
    mockSaveOutputPath.mockResolvedValue('/custom/my-output.pdf');

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    dropHandler!(['/home/user/doc.pdf']);

    await fireEvent.click(await screen.findByRole('button', { name: /save as/i }));
    await vi.waitFor(() => expect(mockSaveOutputPath).toHaveBeenCalled());

    await fireEvent.click(screen.getByRole('button', { name: /run/i }));

    expect(mockCmdMerge).toHaveBeenCalledWith(
      ['/home/user/doc.pdf'],
      '/custom/my-output.pdf'
    );
  });

  it('Run uses default output path when Save As is not used', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    mockCmdMerge.mockResolvedValue('ok');
    // saveOutputPath returns null (user cancelled)
    mockSaveOutputPath.mockResolvedValue(null);

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
