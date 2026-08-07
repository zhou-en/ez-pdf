import { describe, expect, it, vi, beforeEach } from 'vitest';

import { OPERATIONS, runOperation, specFor, stem, type RunInput } from './operations';

const ops = {
  page_count: vi.fn(() => 3),
  info_json: vi.fn(() => JSON.stringify({ page_count: 1, dimensions: [[612, 792]] })),
  merge: vi.fn(() => new Uint8Array([1])),
  split_range: vi.fn(() => new Uint8Array([2])),
  split_each: vi.fn(() => [new Uint8Array([3])]),
  remove: vi.fn(() => new Uint8Array([4])),
  rotate: vi.fn(() => new Uint8Array([5])),
};
const markdown = { to_markdown: vi.fn(() => '# Title\n\nbody') };

vi.mock('./wasm', async () => {
  const actual = await vi.importActual<typeof import('./wasm')>('./wasm');
  return {
    ...actual,
    loadOps: vi.fn(async () => ops),
    loadMarkdown: vi.fn(async () => markdown),
  };
});

function input(overrides: Partial<RunInput> = {}): RunInput {
  return {
    op: 'markdown',
    files: [{ name: 'report.pdf', bytes: new Uint8Array([0]) }],
    selectedPages: [],
    pageBreaks: true,
    degrees: 90,
    ...overrides,
  };
}

beforeEach(() => vi.clearAllMocks());

describe('operation catalogue', () => {
  it('exposes exactly the v1 operations', () => {
    expect(OPERATIONS.map((o) => o.id)).toEqual([
      'markdown',
      'merge',
      'split',
      'remove',
      'rotate',
    ]);
  });

  it('marks only markdown as heavy, since it pulls the 2.4 MB module', () => {
    expect(OPERATIONS.filter((o) => o.heavy).map((o) => o.id)).toEqual(['markdown']);
  });

  it('throws on an unknown operation rather than returning undefined', () => {
    // @ts-expect-error deliberately invalid
    expect(() => specFor('nope')).toThrow(/unknown operation/);
  });
});

describe('stem', () => {
  it('drops a .pdf extension case-insensitively', () => {
    expect(stem('report.pdf')).toBe('report');
    expect(stem('REPORT.PDF')).toBe('REPORT');
    expect(stem('no-extension')).toBe('no-extension');
  });
});

describe('validation', () => {
  it('requires a file', async () => {
    await expect(runOperation(input({ files: [] }))).rejects.toThrow(/at least one PDF/i);
  });

  it('requires a page selection for split, remove and rotate', async () => {
    for (const op of ['split', 'remove', 'rotate'] as const) {
      await expect(runOperation(input({ op, selectedPages: [] }))).rejects.toThrow(
        /select at least one page/i,
      );
    }
  });

  it('requires two files to merge', async () => {
    await expect(runOperation(input({ op: 'merge' }))).rejects.toThrow(/at least two/i);
  });

  it('does not require a page selection for markdown', async () => {
    await expect(runOperation(input({ op: 'markdown' }))).resolves.toBeTruthy();
  });
});

describe('dispatch', () => {
  it('markdown returns text, a .md filename and a preview', async () => {
    const out = await runOperation(input({ op: 'markdown' }));
    expect(markdown.to_markdown).toHaveBeenCalledWith(expect.any(Uint8Array), true);
    expect(out.filename).toBe('report.md');
    expect(out.data).toBe('# Title\n\nbody');
    expect(out.preview).toBe('# Title\n\nbody');
  });

  it('passes the page-breaks flag through', async () => {
    await runOperation(input({ op: 'markdown', pageBreaks: false }));
    expect(markdown.to_markdown).toHaveBeenCalledWith(expect.any(Uint8Array), false);
  });

  it('merge passes every file in order', async () => {
    const files = [
      { name: 'a.pdf', bytes: new Uint8Array([1]) },
      { name: 'b.pdf', bytes: new Uint8Array([2]) },
    ];
    const out = await runOperation(input({ op: 'merge', files }));
    expect(ops.merge).toHaveBeenCalledWith([files[0].bytes, files[1].bytes]);
    expect(out.filename).toBe('a-merged.pdf');
  });

  it('split joins selected pages into a range string', async () => {
    const out = await runOperation(input({ op: 'split', selectedPages: [1, 2, 5] }));
    expect(ops.split_range).toHaveBeenCalledWith(expect.any(Uint8Array), '1,2,5');
    expect(out.filename).toBe('report-split.pdf');
  });

  it('remove passes the selected pages', async () => {
    await runOperation(input({ op: 'remove', selectedPages: [3] }));
    expect(ops.remove).toHaveBeenCalledWith(expect.any(Uint8Array), '3');
  });

  it('rotate passes degrees and pages', async () => {
    await runOperation(input({ op: 'rotate', selectedPages: [2], degrees: 270 }));
    expect(ops.rotate).toHaveBeenCalledWith(expect.any(Uint8Array), 270, '2');
  });

  it('never loads the markdown module for a non-markdown operation', async () => {
    const { loadMarkdown } = await import('./wasm');
    await runOperation(input({ op: 'split', selectedPages: [1] }));
    expect(loadMarkdown).not.toHaveBeenCalled();
  });

  it('surfaces the core error message, which carries the remedy', async () => {
    markdown.to_markdown.mockImplementationOnce(() => {
      throw new Error('no extractable text layer on page(s) [1]; run ocrmypdf first');
    });
    await expect(runOperation(input({ op: 'markdown' }))).rejects.toThrow(/ocrmypdf/);
  });
});
