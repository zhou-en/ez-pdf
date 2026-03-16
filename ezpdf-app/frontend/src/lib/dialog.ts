import { open } from '@tauri-apps/plugin-dialog';

export async function openPdfFiles(): Promise<string[] | null> {
  const result = await open({
    multiple: true,
    filters: [{ name: 'PDF', extensions: ['pdf'] }],
  });
  if (!result) return null;
  return Array.isArray(result) ? result : [result];
}
