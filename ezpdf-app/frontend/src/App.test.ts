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
  cmdInfo: vi.fn().mockResolvedValue({
    page_count: 3,
    dimensions: [[612, 792], [612, 792], [612, 792]],
  }),
  cmdGetMetadata: vi.fn().mockResolvedValue({}),
  cmdListBookmarks: vi.fn().mockResolvedValue([]),
  cmdWatermark: vi.fn(),
  cmdSetMetadata: vi.fn(),
  cmdAddBookmark: vi.fn(),
  cmdExtractImages: vi.fn(),
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

  it('split combined output path ends in .pdf', async () => {
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    const mocks = await import('./lib/tauri');
    vi.mocked(mocks.cmdSplitRange).mockResolvedValue('ok');

    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    await fireEvent.click(screen.getByRole('button', { name: /^split$/i }));
    dropHandler!(['/home/user/doc.pdf']);
    await vi.waitFor(() => expect(screen.getByRole('button', { name: /run split/i })).not.toBeDisabled());

    // select a page tile and run
    const { cmdInfo } = await import('./lib/tauri');
    await vi.waitFor(() => expect(vi.mocked(cmdInfo)).toHaveBeenCalled());
    const tileBtns = screen.getAllByRole('button').filter(b => /^[0-9]+$/.test(b.textContent?.replace(/\s+/g, '') ?? ''));
    await fireEvent.click(tileBtns[0]);

    await fireEvent.click(screen.getByRole('button', { name: /run split/i }));
    await vi.waitFor(() => expect(vi.mocked(mocks.cmdSplitRange)).toHaveBeenCalled());
    const call = vi.mocked(mocks.cmdSplitRange).mock.calls[0];
    expect(call[2]).toMatch(/\.pdf$/i);
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

  // ── Phase 24: grid-driven op tests ────────────────────────────────────────

  async function setupGridOp(opName: string) {
    const { cmdInfo } = await import('./lib/tauri');
    let dropHandler: ((paths: string[]) => void) | undefined;
    mockOnFileDrop.mockImplementation(async (handler) => {
      dropHandler = handler;
      return vi.fn();
    });
    render(App);
    await vi.waitFor(() => expect(dropHandler).toBeDefined());
    await fireEvent.click(screen.getByRole('button', { name: new RegExp(`^${opName}$`, 'i') }));
    dropHandler!(['/home/user/doc.pdf']);
    // Wait for cmdInfo to be called and tiles to appear
    await vi.waitFor(() => expect(vi.mocked(cmdInfo)).toHaveBeenCalledWith('/home/user/doc.pdf'));
    // Wait for 3 tile buttons to appear
    await vi.waitFor(() => expect(screen.getAllByRole('button').filter(b => /^[0-9]+$/.test(b.textContent?.replace(/\s+/g, '') ?? ''))).toHaveLength(3));
    return { cmdInfo };
  }

  it('dropping file on remove op calls cmdInfo and shows page tiles', async () => {
    await setupGridOp('remove');
    // Tiles labeled 1, 2, 3 should be visible
    expect(screen.getByText('1')).toBeInTheDocument();
    expect(screen.getByText('2')).toBeInTheDocument();
    expect(screen.getByText('3')).toBeInTheDocument();
  });

  it('run remove with selected tiles calls cmdRemove with page numbers', async () => {
    const { cmdRemove } = await import('./lib/tauri');
    vi.mocked(cmdRemove).mockResolvedValue('Removed → out.pdf');
    await setupGridOp('remove');

    const tileBtns = screen.getAllByRole('button').filter(b => /^[0-9]+$/.test(b.textContent?.replace(/\s+/g, '') ?? ''));
    await fireEvent.click(tileBtns[0]); // select page 1
    await fireEvent.click(tileBtns[2]); // select page 3

    await fireEvent.click(screen.getByRole('button', { name: /run remove/i }));

    await vi.waitFor(() =>
      expect(vi.mocked(cmdRemove)).toHaveBeenCalledWith('/home/user/doc.pdf', '1,3', expect.any(String))
    );
  });

  it('run reorder calls cmdReorder with tile order from grid', async () => {
    const { cmdReorder } = await import('./lib/tauri');
    vi.mocked(cmdReorder).mockResolvedValue('Reordered → out.pdf');
    await setupGridOp('reorder');

    // Without any drag, tile order is 1,2,3 — reorder should be called with that order
    await fireEvent.click(screen.getByRole('button', { name: /run reorder/i }));

    await vi.waitFor(() =>
      expect(vi.mocked(cmdReorder)).toHaveBeenCalledWith('/home/user/doc.pdf', '1,2,3', expect.any(String))
    );
  });

  it('run rotate with selected tiles calls cmdRotate with page numbers', async () => {
    const { cmdRotate } = await import('./lib/tauri');
    vi.mocked(cmdRotate).mockResolvedValue('Rotated → out.pdf');
    await setupGridOp('rotate');

    const tileBtns = screen.getAllByRole('button').filter(b => /^[0-9]+$/.test(b.textContent?.replace(/\s+/g, '') ?? ''));
    await fireEvent.click(tileBtns[1]); // select page 2

    await fireEvent.click(screen.getByRole('button', { name: /run rotate/i }));

    await vi.waitFor(() =>
      expect(vi.mocked(cmdRotate)).toHaveBeenCalledWith('/home/user/doc.pdf', expect.any(Number), '2', expect.any(String))
    );
  });

  it('run split combined calls cmdSplitRange with selected pages', async () => {
    const { cmdSplitRange } = await import('./lib/tauri');
    vi.mocked(cmdSplitRange).mockResolvedValue('Split → out.pdf');
    await setupGridOp('split');

    const tileBtns = screen.getAllByRole('button').filter(b => /^[0-9]+$/.test(b.textContent?.replace(/\s+/g, '') ?? ''));
    await fireEvent.click(tileBtns[0]); // select page 1
    await fireEvent.click(tileBtns[1]); // select page 2

    await fireEvent.click(screen.getByRole('button', { name: /run split/i }));

    await vi.waitFor(() =>
      expect(vi.mocked(cmdSplitRange)).toHaveBeenCalledWith('/home/user/doc.pdf', '1,2', expect.any(String))
    );
  });

  it('error shown when no pages selected for remove', async () => {
    await setupGridOp('remove');
    await fireEvent.click(screen.getByRole('button', { name: /run remove/i }));
    expect(await screen.findByText(/select at least one page/i)).toBeInTheDocument();
  });

  it('error shown when no pages selected for rotate', async () => {
    await setupGridOp('rotate');
    await fireEvent.click(screen.getByRole('button', { name: /run rotate/i }));
    expect(await screen.findByText(/select at least one page/i)).toBeInTheDocument();
  });

  it('error shown when no pages selected for split', async () => {
    await setupGridOp('split');
    await fireEvent.click(screen.getByRole('button', { name: /run split/i }));
    expect(await screen.findByText(/select at least one page/i)).toBeInTheDocument();
  });
});
