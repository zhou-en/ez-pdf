/* tslint:disable */
/* eslint-disable */

/**
 * Page count, per-page dimensions and metadata, as a JSON string.
 *
 * The page grid needs real page dimensions to render true-aspect tiles.
 */
export function info_json(bytes: Uint8Array): string;

/**
 * Concatenates several PDFs in the given order.
 */
export function merge(docs: Uint8Array[]): Uint8Array;

/**
 * Number of pages in a PDF.
 */
export function page_count(bytes: Uint8Array): number;

/**
 * Deletes the given pages.
 */
export function remove(bytes: Uint8Array, pages: string): Uint8Array;

/**
 * Rotates all pages, or only `pages` when supplied.
 */
export function rotate(bytes: Uint8Array, degrees: number, pages?: string | null): Uint8Array;

/**
 * Bursts a PDF into one document per page.
 */
export function split_each(bytes: Uint8Array): Uint8Array[];

/**
 * Extracts a page range such as `1-5,7`.
 */
export function split_range(bytes: Uint8Array, range: string): Uint8Array;

/**
 * Converts a PDF to Markdown.
 *
 * Only present in the `markdown` feature build — it pulls in `pdf-inspector`,
 * which is ~25x the size of the rest of the module put together.
 */
export function to_markdown(bytes: Uint8Array, page_breaks: boolean): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly info_json: (a: number, b: number) => [number, number, number, number];
    readonly merge: (a: number, b: number) => [number, number, number, number];
    readonly page_count: (a: number, b: number) => [number, number, number];
    readonly remove: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly rotate: (a: number, b: number, c: number, d: number, e: number) => [number, number, number, number];
    readonly split_each: (a: number, b: number) => [number, number, number, number];
    readonly split_range: (a: number, b: number, c: number, d: number) => [number, number, number, number];
    readonly to_markdown: (a: number, b: number, c: number) => [number, number, number, number];
    readonly __wbindgen_exn_store: (a: number) => void;
    readonly __externref_table_alloc: () => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __externref_table_dealloc: (a: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
