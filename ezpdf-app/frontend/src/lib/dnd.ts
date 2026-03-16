import { getCurrentWebview } from '@tauri-apps/api/webview';

export async function onFileDrop(handler: (paths: string[]) => void): Promise<() => void> {
  // lastPaths is scoped per registration to deduplicate Tauri's rapid double-fire
  // without blocking the same file being dropped again later.
  let lastPaths: string[] = [];

  const unlisten = await getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'drop') {
      const paths = event.payload.paths as string[];
      if (JSON.stringify(paths) === JSON.stringify(lastPaths)) return;
      lastPaths = paths;
      handler(paths.filter((p) => p.toLowerCase().endsWith('.pdf')));
      // Clear after a tick so the same file can be dropped again (e.g. after switching tabs)
      setTimeout(() => { lastPaths = []; }, 100);
    }
  });
  return unlisten;
}
