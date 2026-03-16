<script lang="ts">
  import Sidebar from './components/Sidebar.svelte';
  import DropZone from './components/DropZone.svelte';
  import FileList from './components/FileList.svelte';
  import PageGrid from './components/PageGrid.svelte';
  import OptionsPanel from './components/OptionsPanel.svelte';
  import ProgressBar from './components/ProgressBar.svelte';
  import {
    cmdMerge, cmdSplitRange, cmdRemove, cmdRotate, cmdReorder, cmdPageCount,
    cmdGetMetadata, cmdSetMetadata, cmdWatermark, cmdListBookmarks, cmdAddBookmark, cmdExtractImages,
    cmdInfo,
  } from './lib/tauri';
  import type { PdfMetadata, Bookmark } from './lib/tauri';
  import { saveOutputPath, pickOutputDir } from './lib/dialog';

  type Op = 'merge' | 'split' | 'remove' | 'rotate' | 'reorder' | 'metadata' | 'watermark' | 'bookmarks' | 'extract';
  type Status = { type: 'idle' | 'success' | 'error'; message: string };

  interface PageTile {
    pageNum: number;
    width: number;
    height: number;
    selected: boolean;
  }

  const gridOps: Op[] = ['reorder', 'remove', 'rotate', 'split'];

  const ops: Op[] = ['merge', 'split', 'remove', 'rotate', 'reorder', 'metadata', 'watermark', 'bookmarks', 'extract'];

  // Per-operation file lists
  let filesByOp: Record<Op, string[]> = $state({
    merge: [], split: [], remove: [], rotate: [], reorder: [],
    metadata: [], watermark: [], bookmarks: [], extract: [],
  });

  let selectedOp: Op = $state('merge');
  let status: Status = $state({ type: 'idle', message: '' });
  let running = $state(false);

  // Per-operation output path overrides
  let outputOverride: Record<Op, string> = $state({
    merge: '', split: '', remove: '', rotate: '', reorder: '',
    metadata: '', watermark: '', bookmarks: '', extract: '',
  });

  // Grid page tile state (shared across reorder/remove/rotate/split)
  let pageTiles: PageTile[] = $state([]);
  let splitOutputMode: 'combined' | 'individual' = $state('combined');

  // Options state
  let rotateDegrees = $state(90);

  // Metadata op state
  let metaTitle = $state('');
  let metaAuthor = $state('');
  let metaSubject = $state('');
  let metaKeywords = $state('');
  let loadedMeta = $state<PdfMetadata | null>(null);

  // Watermark op state
  let watermarkText = $state('');
  let watermarkFontSize = $state(48);
  let watermarkOpacity = $state(0.3);
  let watermarkPages = $state('');

  // Bookmarks op state
  let bookmarksList = $state<Bookmark[]>([]);
  let bookmarkTitle = $state('');
  let bookmarkPage = $state(1);

  let files = $derived(filesByOp[selectedOp]);

  // Page counts keyed by absolute path (used in merge FileList)
  let pageCounts: Record<string, number> = $state({});

  function addFiles(paths: string[]) {
    filesByOp = { ...filesByOp, [selectedOp]: [...filesByOp[selectedOp], ...paths] };
    for (const p of paths) {
      cmdPageCount(p).then((n) => {
        pageCounts = { ...pageCounts, [p]: n };
      }).catch(() => {/* ignore if count fails */});
    }
    if (paths.length > 0) {
      if (gridOps.includes(selectedOp)) {
        // Load page tiles for grid operations
        cmdInfo(paths[0]).then((info) => {
          pageTiles = info.dimensions.map(([w, h], i) => ({
            pageNum: i + 1,
            width: w,
            height: h,
            selected: false,
          }));
        }).catch(() => {});
      } else if (selectedOp === 'metadata') {
        cmdGetMetadata(paths[0]).then((m) => {
          loadedMeta = m;
          metaTitle = m.title ?? '';
          metaAuthor = m.author ?? '';
          metaSubject = m.subject ?? '';
          metaKeywords = m.keywords ?? '';
        }).catch(() => {});
      } else if (selectedOp === 'bookmarks') {
        cmdListBookmarks(paths[0]).then((bms) => {
          bookmarksList = bms;
        }).catch(() => {});
      }
    }
  }

  function removeFile(index: number) {
    filesByOp = {
      ...filesByOp,
      [selectedOp]: filesByOp[selectedOp].filter((_, i) => i !== index),
    };
  }

  function reorderFiles(from: number, to: number) {
    const list = [...filesByOp[selectedOp]];
    const [moved] = list.splice(from, 1);
    list.splice(to, 0, moved);
    filesByOp = { ...filesByOp, [selectedOp]: list };
  }

  function basename(path: string): string {
    return path.replace(/^.*[/\\]/, '');
  }

  function dirname(path: string): string {
    const idx = path.lastIndexOf('/');
    return idx >= 0 ? path.slice(0, idx + 1) : '';
  }

  function stem(path: string): string {
    return basename(path).replace(/\.pdf$/i, '');
  }

  const opSuffix: Record<Op, string> = {
    merge: 'merged',
    split: 'split',
    remove: 'removed',
    rotate: 'rotated',
    reorder: 'reordered',
    metadata: 'meta',
    watermark: 'watermarked',
    bookmarks: 'bookmarked',
    extract: 'images',
  };

  function defaultOutput(op: Op): string {
    if (files.length === 0) return '';
    const dir = dirname(files[0]);
    const base = stem(files[0]);
    if (op === 'extract') {
      return `${dir}${base}-images`;
    }
    return `${dir}${base}-${opSuffix[op]}.pdf`;
  }

  function resolvedOutput(op: Op): string {
    return outputOverride[op] || defaultOutput(op);
  }

  async function handleSaveAs() {
    const needsDir =
      (selectedOp === 'split' && splitOutputMode === 'individual') ||
      selectedOp === 'extract';
    const def = defaultOutput(selectedOp);
    const picked = needsDir
      ? await pickOutputDir(def)
      : await saveOutputPath(def);
    if (picked) {
      outputOverride = { ...outputOverride, [selectedOp]: picked };
    }
  }

  async function run() {
    if (files.length === 0) {
      status = { type: 'error', message: 'Add at least one PDF file.' };
      return;
    }

    // Grid-op validations
    if (['remove', 'rotate', 'split'].includes(selectedOp)) {
      if (pageTiles.filter((t) => t.selected).length === 0) {
        status = { type: 'error', message: 'Select at least one page.' };
        return;
      }
    }

    if (selectedOp === 'watermark' && watermarkText.trim() === '') {
      status = { type: 'error', message: 'Enter watermark text.' };
      return;
    }
    if (selectedOp === 'bookmarks' && bookmarkTitle.trim() === '') {
      status = { type: 'error', message: 'Enter a bookmark title.' };
      return;
    }

    running = true;
    status = { type: 'idle', message: '' };
    try {
      let msg: string;
      const out = resolvedOutput(selectedOp);

      if (selectedOp === 'merge') {
        msg = await cmdMerge(files, out);
      } else if (selectedOp === 'reorder') {
        const order = pageTiles.map((t) => t.pageNum).join(',');
        msg = await cmdReorder(files[0], order, out);
      } else if (selectedOp === 'remove') {
        const pages = pageTiles.filter((t) => t.selected).map((t) => t.pageNum).join(',');
        msg = await cmdRemove(files[0], pages, out);
      } else if (selectedOp === 'rotate') {
        const pages = pageTiles.filter((t) => t.selected).map((t) => t.pageNum).join(',');
        msg = await cmdRotate(files[0], rotateDegrees, pages || null, out);
      } else if (selectedOp === 'split') {
        const selected = pageTiles.filter((t) => t.selected).map((t) => t.pageNum);
        if (splitOutputMode === 'individual') {
          const results: string[] = [];
          for (const p of selected) {
            const pageOut = out.replace(/\.pdf$/i, '') + `-page-${p}.pdf`;
            results.push(await cmdSplitRange(files[0], String(p), pageOut));
          }
          msg = results.join('\n');
        } else {
          msg = await cmdSplitRange(files[0], selected.join(','), out);
        }
      } else if (selectedOp === 'metadata') {
        msg = await cmdSetMetadata(
          files[0], out,
          metaTitle || null, metaAuthor || null,
          metaSubject || null, metaKeywords || null,
          null, null,
        );
      } else if (selectedOp === 'watermark') {
        msg = await cmdWatermark(
          files[0], watermarkText, watermarkFontSize, watermarkOpacity,
          watermarkPages || null, out,
        );
      } else if (selectedOp === 'bookmarks') {
        msg = await cmdAddBookmark(files[0], bookmarkTitle, bookmarkPage, out);
        bookmarksList = await cmdListBookmarks(out);
      } else {
        // extract
        msg = await cmdExtractImages(files[0], out);
      }
      status = { type: 'success', message: msg };
    } catch (err) {
      status = { type: 'error', message: String(err) };
    } finally {
      running = false;
    }
  }
</script>

<div class="app">
  <Sidebar {selectedOp} onopSelected={(op) => {
    selectedOp = op;
    status = { type: 'idle', message: '' };
  }} />

  <main>
    <DropZone onfilesAdded={addFiles} />

    {#if files.length > 0}
      {#if selectedOp === 'merge'}
        <FileList
          {files}
          {pageCounts}
          onremove={removeFile}
          onreorder={reorderFiles}
        />
      {:else if gridOps.includes(selectedOp)}
        <PageGrid
          tiles={pageTiles}
          mode={selectedOp === 'reorder' ? 'reorder' : 'select'}
          ontiles={(t) => (pageTiles = t)}
        />
      {/if}

      <div class="output-row">
        <span class="output-path">{resolvedOutput(selectedOp)}</span>
        <button class="save-as-btn" onclick={handleSaveAs} aria-label="Save as">
          Save As…
        </button>
      </div>
    {/if}

    <OptionsPanel
      op={selectedOp}
      bind:splitOutputMode
      bind:rotateDegrees
      bind:metaTitle
      bind:metaAuthor
      bind:metaSubject
      bind:metaKeywords
      bind:loadedMeta
      bind:watermarkText
      bind:watermarkFontSize
      bind:watermarkOpacity
      bind:watermarkPages
      bind:bookmarksList
      bind:bookmarkTitle
      bind:bookmarkPage
    />

    <button class="run-btn" onclick={run} disabled={files.length === 0 || running}>
      {#if running}
        Running…
      {:else}
        Run {selectedOp.charAt(0).toUpperCase() + selectedOp.slice(1)}
      {/if}
    </button>

    <ProgressBar visible={running} />

    {#if status.type !== 'idle'}
      <p class="status {status.type}">{status.message}</p>
    {/if}
  </main>
</div>

<style>
  :root {
    --bg: #ffffff;
    --bg-sidebar: #1e293b;
    --text: #111827;
    --text-muted: #6b7280;
    --border: #e5e7eb;
    --file-item-bg: #f3f4f6;
    --run-btn: #2563eb;
    --run-btn-hover: #1d4ed8;
    --run-btn-disabled: #93c5fd;
  }

  @media (prefers-color-scheme: dark) {
    :root {
      --bg: #111827;
      --bg-sidebar: #0f172a;
      --text: #f9fafb;
      --text-muted: #9ca3af;
      --border: #374151;
      --file-item-bg: #1f2937;
      --run-btn: #3b82f6;
      --run-btn-hover: #2563eb;
      --run-btn-disabled: #1e3a5f;
    }
  }

  .app {
    display: flex;
    height: 100vh;
    font-family: system-ui, sans-serif;
    background: var(--bg);
    color: var(--text);
  }

  main {
    flex: 1;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
    overflow-y: auto;
  }

  .output-row {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.8rem;
  }

  .output-path {
    flex: 1;
    color: var(--text-muted);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .save-as-btn {
    background: none;
    border: 1px solid var(--border);
    color: var(--text);
    border-radius: 4px;
    padding: 0.2rem 0.6rem;
    font-size: 0.8rem;
    cursor: pointer;
    white-space: nowrap;
  }

  .save-as-btn:hover {
    background: var(--file-item-bg);
  }

  .run-btn {
    align-self: flex-start;
    padding: 0.5rem 1.5rem;
    background: var(--run-btn);
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
  }

  .run-btn:disabled {
    background: var(--run-btn-disabled);
    cursor: not-allowed;
  }

  .run-btn:not(:disabled):hover {
    background: var(--run-btn-hover);
  }

  .status {
    padding: 0.5rem 0.75rem;
    border-radius: 4px;
    font-size: 0.875rem;
    margin: 0;
  }

  .status.success {
    background: #dcfce7;
    color: #166534;
  }

  .status.error {
    background: #fee2e2;
    color: #991b1b;
  }
</style>
