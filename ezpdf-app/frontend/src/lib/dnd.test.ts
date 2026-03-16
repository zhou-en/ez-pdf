import { describe, it, expect, vi, beforeEach } from 'vitest';

// Mock the Tauri webview API before importing dnd
const mockOnDragDropEvent = vi.fn();
vi.mock('@tauri-apps/api/webview', () => ({
  getCurrentWebview: () => ({ onDragDropEvent: mockOnDragDropEvent }),
}));

// Import AFTER mock so the module picks up the mock
const { onFileDrop } = await import('./dnd');

beforeEach(() => {
  mockOnDragDropEvent.mockReset();
  mockOnDragDropEvent.mockResolvedValue(vi.fn());
});

function makeDropEvent(paths: string[]) {
  return { payload: { type: 'drop', paths } };
}

describe('onFileDrop', () => {
  it('calls handler with pdf paths on drop', async () => {
    let listener: ((e: ReturnType<typeof makeDropEvent>) => void) | undefined;
    mockOnDragDropEvent.mockImplementation(async (fn) => { listener = fn; return vi.fn(); });

    const handler = vi.fn();
    await onFileDrop(handler);

    listener!(makeDropEvent(['/a/doc.pdf']));
    expect(handler).toHaveBeenCalledWith(['/a/doc.pdf']);
  });

  it('filters out non-pdf files', async () => {
    let listener: ((e: ReturnType<typeof makeDropEvent>) => void) | undefined;
    mockOnDragDropEvent.mockImplementation(async (fn) => { listener = fn; return vi.fn(); });

    const handler = vi.fn();
    await onFileDrop(handler);

    listener!(makeDropEvent(['/a/doc.pdf', '/a/image.png']));
    expect(handler).toHaveBeenCalledWith(['/a/doc.pdf']);
  });

  it('fires again with the same paths after switching handlers (tab switch scenario)', async () => {
    let listener: ((e: ReturnType<typeof makeDropEvent>) => void) | undefined;
    mockOnDragDropEvent.mockImplementation(async (fn) => { listener = fn; return vi.fn(); });

    const handler1 = vi.fn();
    await onFileDrop(handler1);
    listener!(makeDropEvent(['/a/doc.pdf']));
    expect(handler1).toHaveBeenCalledTimes(1);

    // Simulate switching op: new handler registered (second onFileDrop call)
    const handler2 = vi.fn();
    await onFileDrop(handler2);
    listener!(makeDropEvent(['/a/doc.pdf'])); // same file, new tab
    expect(handler2).toHaveBeenCalledTimes(1);
  });

  it('deduplicates rapid duplicate fires within the same handler', async () => {
    let listener: ((e: ReturnType<typeof makeDropEvent>) => void) | undefined;
    mockOnDragDropEvent.mockImplementation(async (fn) => { listener = fn; return vi.fn(); });

    const handler = vi.fn();
    await onFileDrop(handler);

    // Tauri sometimes fires twice for one drop
    listener!(makeDropEvent(['/a/doc.pdf']));
    listener!(makeDropEvent(['/a/doc.pdf']));
    expect(handler).toHaveBeenCalledTimes(1);
  });
});
