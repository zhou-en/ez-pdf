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
