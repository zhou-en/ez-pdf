import { invoke } from '@tauri-apps/api/core';

export async function cmdMerge(inputs: string[], output: string): Promise<string> {
  return invoke<string>('cmd_merge', { inputs, output });
}

export async function cmdSplitRange(input: string, range: string, output: string): Promise<string> {
  return invoke<string>('cmd_split_range', { input, range, output });
}

export async function cmdSplitEach(input: string, outputDir: string): Promise<string> {
  return invoke<string>('cmd_split_each', { input, outputDir });
}

export async function cmdRemove(input: string, pages: string, output: string): Promise<string> {
  return invoke<string>('cmd_remove', { input, pages, output });
}

export async function cmdRotate(
  input: string,
  degrees: number,
  pages: string | null,
  output: string
): Promise<string> {
  return invoke<string>('cmd_rotate', { input, degrees, pages, output });
}

export async function cmdReorder(input: string, order: string, output: string): Promise<string> {
  return invoke<string>('cmd_reorder', { input, order, output });
}

export async function cmdPageCount(input: string): Promise<number> {
  return invoke<number>('cmd_page_count', { input });
}

export interface PdfMetadata {
  title: string | null;
  author: string | null;
  subject: string | null;
  keywords: string | null;
  creator: string | null;
  producer: string | null;
}

export interface Bookmark {
  title: string;
  page: number;
  level: number;
}

export async function cmdGetMetadata(input: string): Promise<PdfMetadata> {
  return invoke<PdfMetadata>('cmd_get_metadata', { input });
}

export async function cmdSetMetadata(
  input: string,
  output: string,
  title: string | null,
  author: string | null,
  subject: string | null,
  keywords: string | null,
  creator: string | null,
  producer: string | null,
): Promise<string> {
  return invoke<string>('cmd_set_metadata', { input, output, title, author, subject, keywords, creator, producer });
}

export async function cmdWatermark(
  input: string,
  text: string,
  fontSize: number,
  opacity: number,
  pages: string | null,
  output: string,
): Promise<string> {
  return invoke<string>('cmd_watermark', { input, text, fontSize, opacity, pages, output });
}

export async function cmdListBookmarks(input: string): Promise<Bookmark[]> {
  return invoke<Bookmark[]>('cmd_list_bookmarks', { input });
}

export async function cmdAddBookmark(
  input: string,
  title: string,
  page: number,
  output: string,
): Promise<string> {
  return invoke<string>('cmd_add_bookmark', { input, title, page, output });
}

export async function cmdExtractImages(input: string, outputDir: string): Promise<string> {
  return invoke<string>('cmd_extract_images', { input, outputDir });
}

export interface PdfInfo {
  page_count: number;
  dimensions: [number, number][];
}

export async function cmdInfo(input: string): Promise<PdfInfo> {
  return invoke<PdfInfo>('cmd_info', { input });
}
