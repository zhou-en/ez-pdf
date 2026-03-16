import { open, save } from '@tauri-apps/plugin-dialog';

export async function openPdfFiles(): Promise<string[] | null> {
  const result = await open({
    multiple: true,
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
  });
  if (!result) return null;
  return Array.isArray(result) ? result : [result];
}

export async function saveOutputPath(defaultPath: string): Promise<string | null> {
  return save({
    defaultPath,
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
  });
}

export async function pickOutputDir(defaultPath: string): Promise<string | null> {
  const result = await open({ directory: true, defaultPath });
  if (!result) return null;
  return Array.isArray(result) ? result[0] : result;
}
