'use client';

import { useCallback, useEffect, useRef, useState } from 'react';

import { DropZone } from './DropZone';
import { PageGrid, tilesFromDimensions, type PageTile } from './PageGrid';
import { Button, Card, Chip, Keycap, PillTab } from './ui';
import {
  OPERATIONS,
  readInfo,
  runOperation,
  specFor,
  type OpId,
  type OpResult,
} from '@/lib/operations';
import { formatBytes } from '@/lib/format';
import { messageOf } from '@/lib/wasm';

interface LoadedFile {
  name: string;
  size: number;
  bytes: Uint8Array;
}

type Status =
  | { kind: 'idle' }
  | { kind: 'busy'; message: string }
  | { kind: 'error'; message: string };

export function Converter({ userId }: { userId: string | null }) {
  const signedIn = userId !== null;
  const [op, setOp] = useState<OpId>('markdown');
  const [files, setFiles] = useState<LoadedFile[]>([]);
  const [tiles, setTiles] = useState<PageTile[]>([]);
  const [pageBreaks, setPageBreaks] = useState(true);
  const [degrees, setDegrees] = useState(90);
  const [status, setStatus] = useState<Status>({ kind: 'idle' });
  const [result, setResult] = useState<OpResult | null>(null);
  const [saved, setSaved] = useState(false);
  const dropRef = useRef<HTMLDivElement>(null);

  const spec = specFor(op);

  const addFiles = useCallback(
    async (incoming: File[]) => {
      setResult(null);
      setSaved(false);
      setStatus({ kind: 'idle' });

      const loaded: LoadedFile[] = await Promise.all(
        incoming.map(async (f) => ({
          name: f.name,
          size: f.size,
          bytes: new Uint8Array(await f.arrayBuffer()),
        })),
      );

      const next = spec.multiFile ? [...files, ...loaded] : loaded.slice(0, 1);
      setFiles(next);

      if (specFor(op).needsPageSelection && next[0]) {
        try {
          const info = await readInfo(next[0].bytes);
          setTiles(tilesFromDimensions(info.dimensions));
        } catch (err) {
          setStatus({ kind: 'error', message: messageOf(err) });
          setTiles([]);
        }
      } else {
        setTiles([]);
      }
    },
    [files, op, spec.multiFile],
  );

  // Switching operations keeps the files but resets everything derived from them.
  function selectOp(next: OpId) {
    setOp(next);
    setResult(null);
    setSaved(false);
    setStatus({ kind: 'idle' });
    setTiles([]);
    if (!specFor(next).multiFile) setFiles((f) => f.slice(0, 1));
  }

  useEffect(() => {
    const first = files[0];
    if (!first || !specFor(op).needsPageSelection || tiles.length > 0) return;
    readInfo(first.bytes)
      .then((info) => setTiles(tilesFromDimensions(info.dimensions)))
      .catch((err) => setStatus({ kind: 'error', message: messageOf(err) }));
  }, [op, files, tiles.length]);

  const run = useCallback(async () => {
    setResult(null);
    setSaved(false);
    setStatus({
      kind: 'busy',
      message: spec.heavy ? 'Loading the Markdown engine…' : 'Working…',
    });
    try {
      const out = await runOperation({
        op,
        files,
        selectedPages: tiles.filter((t) => t.selected).map((t) => t.pageNum),
        pageBreaks,
        degrees,
      });
      setResult(out);
      setStatus({ kind: 'idle' });
    } catch (err) {
      setStatus({ kind: 'error', message: messageOf(err) });
    }
  }, [op, files, tiles, pageBreaks, degrees, spec.heavy]);

  // ⌘O / Ctrl+O opens the picker; ⏎ converts. The keycaps advertise both.
  useEffect(() => {
    function onKey(e: KeyboardEvent) {
      if ((e.metaKey || e.ctrlKey) && e.key.toLowerCase() === 'o') {
        e.preventDefault();
        dropRef.current?.querySelector<HTMLButtonElement>('button')?.click();
      } else if (e.key === 'Enter' && !e.metaKey && files.length > 0) {
        const tag = (e.target as HTMLElement | null)?.tagName;
        if (tag !== 'INPUT' && tag !== 'BUTTON') void run();
      }
    }
    window.addEventListener('keydown', onKey);
    return () => window.removeEventListener('keydown', onKey);
  }, [files.length, run]);

  function download() {
    if (!result) return;
    const blob =
      typeof result.data === 'string'
        ? new Blob([result.data], { type: 'text/markdown' })
        : new Blob([result.data as BlobPart], { type: 'application/pdf' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = result.filename;
    a.click();
    URL.revokeObjectURL(url);
  }

  async function save() {
    if (!result || files.length === 0) return;
    if (!userId) {
      // Resume the save after Google sign-in rather than losing the work.
      sessionStorage.setItem('ezpdf:resume-save', '1');
      const { signIn } = await import('next-auth/react');
      await signIn('google');
      return;
    }
    setStatus({ kind: 'busy', message: 'Saving…' });
    try {
      const { saveToLibrary } = await import('@/lib/save');
      await saveToLibrary({ op, userId, source: files[0], result });
      setSaved(true);
      setStatus({ kind: 'idle' });
    } catch (err) {
      setStatus({ kind: 'error', message: messageOf(err) });
    }
  }

  const busy = status.kind === 'busy';

  return (
    <Card className="overflow-hidden">
      {/* command-palette-card header strip */}
      <div className="flex items-center justify-between border-b border-hairline px-4 py-2.5">
        <div className="flex gap-1.5" aria-hidden>
          <span className="size-2.5 rounded-full bg-hairline-strong" />
          <span className="size-2.5 rounded-full bg-hairline-strong" />
          <span className="size-2.5 rounded-full bg-hairline-strong" />
        </div>
        <span className="flex items-center gap-1.5 text-xs text-mute">
          <Keycap>⌘O</Keycap> to open
        </span>
      </div>

      <div className="flex flex-col gap-4 p-4 sm:p-6">
        <div className="flex flex-wrap gap-1" role="tablist" aria-label="Operation">
          {OPERATIONS.map((o) => (
            <PillTab key={o.id} active={o.id === op} onClick={() => selectOp(o.id)}>
              {o.label}
            </PillTab>
          ))}
        </div>
        <p className="-mt-2 text-sm text-mute">{spec.blurb}</p>

        <div ref={dropRef}>
          <DropZone multiple={spec.multiFile} onFiles={addFiles} />
        </div>

        {files.length > 0 && (
          <ul className="flex flex-col gap-1.5">
            {files.map((f, i) => (
              <li
                key={`${f.name}-${i}`}
                className="flex items-center gap-3 rounded-md bg-card px-3 py-2 text-sm"
              >
                <span className="flex-1 truncate">{f.name}</span>
                <span className="shrink-0 text-xs text-mute">{formatBytes(f.size)}</span>
                <button
                  type="button"
                  aria-label={`Remove ${f.name}`}
                  className="flex size-11 shrink-0 items-center justify-center rounded-xs text-lg text-mute hover:text-accent-red pointer-fine:size-6"
                  onClick={() => {
                    setFiles((prev) => prev.filter((_, j) => j !== i));
                    setTiles([]);
                    setResult(null);
                  }}
                >
                  ×
                </button>
              </li>
            ))}
          </ul>
        )}

        {spec.needsPageSelection && tiles.length > 0 && (
          <PageGrid tiles={tiles} onChange={setTiles} />
        )}

        {op === 'markdown' && (
          <label className="flex items-center gap-2 text-sm text-body">
            <input
              type="checkbox"
              checked={pageBreaks}
              onChange={(e) => setPageBreaks(e.target.checked)}
              className="accent-accent-blue"
            />
            Insert page break separators
          </label>
        )}

        {op === 'rotate' && (
          <label className="flex flex-col gap-1.5 text-sm">
            <span className="text-xs uppercase tracking-wide text-mute">Degrees</span>
            <select
              value={degrees}
              onChange={(e) => setDegrees(Number(e.target.value))}
              className="h-11 w-48 rounded-md border border-hairline bg-elevated px-2 text-ink pointer-fine:h-9"
            >
              <option value={90}>90° clockwise</option>
              <option value={180}>180°</option>
              <option value={270}>270° clockwise</option>
              <option value={-90}>90° counter-clockwise</option>
            </select>
          </label>
        )}

        <div className="flex items-center gap-3">
          <Button onClick={run} disabled={files.length === 0 || busy}>
            {busy ? status.message : `Run ${spec.label}`}
          </Button>
          {files.length > 0 && !busy && (
            <span className="flex items-center gap-1.5 text-xs text-mute">
              <Keycap>⏎</Keycap>
            </span>
          )}
        </div>

        <div aria-live="polite" className="empty:hidden">
          {status.kind === 'error' && <Chip tone="danger">{status.message}</Chip>}
          {saved && <Chip tone="success">Saved to your library.</Chip>}
        </div>

        {result && (
          <div className="rounded-md border border-hairline bg-elevated p-4">
            <div className="mb-3 flex items-baseline justify-between gap-3">
              <span className="truncate font-medium">{result.filename}</span>
              <span className="shrink-0 text-xs text-mute">
                {formatBytes(
                  typeof result.data === 'string' ? result.data.length : result.data.byteLength,
                )}
              </span>
            </div>

            {result.preview && (
              <pre className="mb-3 max-h-48 overflow-auto rounded-sm bg-canvas p-3 font-mono text-xs leading-relaxed text-body">
                {result.preview.slice(0, 1500)}
              </pre>
            )}

            <div className="flex flex-wrap gap-2">
              <Button onClick={download}>Download</Button>
              <Button variant="tertiary" onClick={save} disabled={saved || busy}>
                {signedIn ? 'Save to library' : 'Sign in to save'}
              </Button>
            </div>
          </div>
        )}
      </div>
    </Card>
  );
}
