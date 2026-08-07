/**
 * Lazy loaders for the two wasm packages.
 *
 * The split is the whole point: `ops` is ~140 KB gzipped and covers
 * merge/split/remove/rotate/info, while `markdown` is ~2.4 MB because
 * pdf-inspector embeds glyph tables and CMaps. Loading them separately means
 * someone merging two PDFs never downloads the markdown engine.
 *
 * Both modules are cached after first load, so switching operations back and
 * forth costs nothing.
 */

export interface PdfInfo {
  page_count: number;
  dimensions: [number, number][];
  title: string | null;
  author: string | null;
  subject: string | null;
  keywords: string | null;
  creator: string | null;
  producer: string | null;
}

interface OpsModule {
  page_count(bytes: Uint8Array): number;
  info_json(bytes: Uint8Array): string;
  merge(docs: Uint8Array[]): Uint8Array;
  split_range(bytes: Uint8Array, range: string): Uint8Array;
  split_each(bytes: Uint8Array): Uint8Array[];
  remove(bytes: Uint8Array, pages: string): Uint8Array;
  rotate(bytes: Uint8Array, degrees: number, pages?: string): Uint8Array;
}

interface MarkdownModule {
  to_markdown(bytes: Uint8Array, pageBreaks: boolean): string;
}

let opsPromise: Promise<OpsModule> | null = null;
let markdownPromise: Promise<MarkdownModule> | null = null;

export function loadOps(): Promise<OpsModule> {
  opsPromise ??= import('@/wasm/ops/ezpdf_ops.js').then(async (m) => {
    await m.default();
    return m as unknown as OpsModule;
  });
  return opsPromise;
}

export function loadMarkdown(): Promise<MarkdownModule> {
  markdownPromise ??= import('@/wasm/markdown/ezpdf_markdown.js').then(async (m) => {
    await m.default();
    return m as unknown as MarkdownModule;
  });
  return markdownPromise;
}

/** True once the markdown module is in memory — drives the "loading engine" hint. */
export function isMarkdownLoaded(): boolean {
  return markdownPromise !== null;
}

/**
 * wasm-bindgen throws plain `Error`s carrying the message `EzPdfError`
 * produced, which already includes a remedy (e.g. `NoTextLayer` names
 * `ocrmypdf`). Surface that text rather than a generic failure message.
 */
export function messageOf(err: unknown): string {
  if (err instanceof Error && err.message) return err.message;
  if (typeof err === 'string') return err;
  return 'Something went wrong converting this file.';
}
