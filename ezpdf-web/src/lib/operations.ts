/**
 * The operation catalogue. Everything the converter UI needs to render and run
 * an operation lives here, so adding one is a single entry rather than edits
 * scattered across a sidebar array, an options panel and a dispatch chain —
 * which is how the desktop app grew.
 */
import { loadMarkdown, loadOps } from './wasm';

export type OpId = 'markdown' | 'merge' | 'split' | 'remove' | 'rotate';

export interface OpResult {
  /** Bytes for a PDF result, or the text of a markdown result. */
  data: Uint8Array | string;
  filename: string;
  /** Rendered inline when present. */
  preview?: string;
}

export interface OpSpec {
  id: OpId;
  label: string;
  blurb: string;
  /** Accepts more than one input file (merge). */
  multiFile: boolean;
  /** Needs the page grid, and at least one page selected. */
  needsPageSelection: boolean;
  /** Downloads the heavy markdown wasm module. */
  heavy: boolean;
}

export const OPERATIONS: OpSpec[] = [
  {
    id: 'markdown',
    label: 'Markdown',
    blurb: 'Extract headings, lists and tables as Markdown.',
    multiFile: false,
    needsPageSelection: false,
    heavy: true,
  },
  {
    id: 'merge',
    label: 'Merge',
    blurb: 'Combine several PDFs into one, in the order listed.',
    multiFile: true,
    needsPageSelection: false,
    heavy: false,
  },
  {
    id: 'split',
    label: 'Split',
    blurb: 'Keep only the pages you select.',
    multiFile: false,
    needsPageSelection: true,
    heavy: false,
  },
  {
    id: 'remove',
    label: 'Remove',
    blurb: 'Delete the pages you select.',
    multiFile: false,
    needsPageSelection: true,
    heavy: false,
  },
  {
    id: 'rotate',
    label: 'Rotate',
    blurb: 'Turn the pages you select.',
    multiFile: false,
    needsPageSelection: true,
    heavy: false,
  },
];

export function specFor(id: OpId): OpSpec {
  const spec = OPERATIONS.find((o) => o.id === id);
  if (!spec) throw new Error(`unknown operation: ${id}`);
  return spec;
}

/** Strips the extension so results can be named after their source. */
export function stem(filename: string): string {
  return filename.replace(/\.pdf$/i, '');
}

export interface RunInput {
  op: OpId;
  files: { name: string; bytes: Uint8Array }[];
  /** 1-indexed pages, for the operations that need a selection. */
  selectedPages: number[];
  pageBreaks: boolean;
  degrees: number;
}

/**
 * Runs an operation. Validation errors are thrown as plain `Error`s with
 * user-facing text; wasm errors already carry the core's remedy wording.
 */
export async function runOperation(input: RunInput): Promise<OpResult> {
  const { op, files, selectedPages, pageBreaks, degrees } = input;

  if (files.length === 0) throw new Error('Add at least one PDF first.');

  const spec = specFor(op);
  if (spec.needsPageSelection && selectedPages.length === 0) {
    throw new Error('Select at least one page.');
  }
  if (op === 'merge' && files.length < 2) {
    throw new Error('Merge needs at least two PDFs.');
  }

  const first = files[0];
  const pages = selectedPages.join(',');

  if (op === 'markdown') {
    const wasm = await loadMarkdown();
    const md = wasm.to_markdown(first.bytes, pageBreaks);
    return { data: md, filename: `${stem(first.name)}.md`, preview: md };
  }

  const wasm = await loadOps();

  switch (op) {
    case 'merge':
      return {
        data: wasm.merge(files.map((f) => f.bytes)),
        filename: `${stem(first.name)}-merged.pdf`,
      };
    case 'split':
      return {
        data: wasm.split_range(first.bytes, pages),
        filename: `${stem(first.name)}-split.pdf`,
      };
    case 'remove':
      return {
        data: wasm.remove(first.bytes, pages),
        filename: `${stem(first.name)}-removed.pdf`,
      };
    case 'rotate':
      return {
        data: wasm.rotate(first.bytes, degrees, pages),
        filename: `${stem(first.name)}-rotated.pdf`,
      };
  }
}

/** Reads page dimensions so the grid can draw true-aspect tiles. */
export async function readInfo(bytes: Uint8Array) {
  const wasm = await loadOps();
  return JSON.parse(wasm.info_json(bytes)) as import('./wasm').PdfInfo;
}
