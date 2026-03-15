import { getCurrentWebview } from '@tauri-apps/api/webview';

let lastPaths: string[] = [];

export async function onFileDrop(handler: (paths: string[]) => void): Promise<() => void> {
  const unlisten = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'drop') {
      const paths = event.payload.paths as string[];
      if (JSON.stringify(paths) !== JSON.stringify(lastPaths)) {
        lastPaths = paths;
        handler(paths.filter((p) => p.toLowerCase().endsWith('.pdf')));
      }
    }
  });
  return unlisten;
}
