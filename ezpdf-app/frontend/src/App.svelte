<script lang="ts">
  import Sidebar from './components/Sidebar.svelte';
  import DropZone from './components/DropZone.svelte';
  import FileList from './components/FileList.svelte';
  import OptionsPanel from './components/OptionsPanel.svelte';
  import ProgressBar from './components/ProgressBar.svelte';
  import { cmdMerge, cmdSplitRange, cmdSplitEach, cmdRemove, cmdRotate, cmdReorder, cmdPageCount } from './lib/tauri';
  import { saveOutputPath, pickOutputDir } from './lib/dialog';

  type Op = 'merge' | 'split' | 'remove' | 'rotate' | 'reorder';
  type Status = { type: 'idle' | 'success' | 'error'; message: string };

  const ops: Op[] = ['merge', 'split', 'remove', 'rotate', 'reorder'];

  // Per-operation file lists
  let filesByOp: Record<Op, string[]> = $state({
    merge: [], split: [], remove: [], rotate: [], reorder: [],
  });

  let selectedOp: Op = $state('merge');
  let status: Status = $state({ type: 'idle', message: '' });
  let running = $state(false);

  // Per-operation output path overrides
  let outputOverride: Record<Op, string> = $state({
    merge: '', split: '', remove: '', rotate: '', reorder: '',
  });

  // Options state
  let splitMode = $state('range');
  let splitRange = $state('');
  let removePages = $state('');
  let rotateDegrees = $state(90);
  let rotatePages = $state('');
  let reorderOrder = $state('');

  let files = $derived(filesByOp[selectedOp]);

  // Page counts keyed by absolute path
  let pageCounts: Record<string, number> = $state({});

  function addFiles(paths: string[]) {
    filesByOp = { ...filesByOp, [selectedOp]: [...filesByOp[selectedOp], ...paths] };
    for (const p of paths) {
      cmdPageCount(p).then((n) => {
        pageCounts = { ...pageCounts, [p]: n };
      }).catch(() => {/* ignore if count fails */});
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
  };

  function defaultOutput(op: Op): string {
    if (files.length === 0) return '';
    const dir = dirname(files[0]);
    const base = stem(files[0]);
    if (op === 'split' && splitMode === 'burst') {
      return `${dir}${base}-pages`;
    }
    return `${dir}${base}-${opSuffix[op]}.pdf`;
  }

  function resolvedOutput(op: Op): string {
    return outputOverride[op] || defaultOutput(op);
  }

  async function handleSaveAs() {
    const isBurst = selectedOp === 'split' && splitMode === 'burst';
    const def = defaultOutput(selectedOp);
    const picked = isBurst
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
    if (selectedOp === 'split' && splitMode === 'range' && splitRange.trim() === '') {
      status = { type: 'error', message: 'Enter a page range (e.g. 1-3) or switch to Burst all pages.' };
      return;
    }
    if (selectedOp === 'remove' && removePages.trim() === '') {
      status = { type: 'error', message: 'Enter page numbers to remove (e.g. 2,4-6).' };
      return;
    }
    if (selectedOp === 'reorder' && reorderOrder.trim() === '') {
      status = { type: 'error', message: 'Enter the new page order (e.g. 3,1,2).' };
      return;
    }
    running = true;
    status = { type: 'idle', message: '' };
    try {
      let msg: string;
      const out = resolvedOutput(selectedOp);
      if (selectedOp === 'merge') {
        msg = await cmdMerge(files, out);
      } else if (selectedOp === 'split') {
        if (splitMode === 'burst') {
          msg = await cmdSplitEach(files[0], out);
        } else {
          msg = await cmdSplitRange(files[0], splitRange, out);
        }
      } else if (selectedOp === 'remove') {
        msg = await cmdRemove(files[0], removePages, out);
      } else if (selectedOp === 'rotate') {
        msg = await cmdRotate(files[0], rotateDegrees, rotatePages || null, out);
      } else {
        msg = await cmdReorder(files[0], reorderOrder, out);
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
      <FileList
        {files}
        {pageCounts}
        onremove={removeFile}
        onreorder={reorderFiles}
      />

      <div class="output-row">
        <span class="output-path">{resolvedOutput(selectedOp)}</span>
        <button class="save-as-btn" onclick={handleSaveAs} aria-label="Save as">
          Save As…
        </button>
      </div>
    {/if}

    <OptionsPanel
      op={selectedOp}
      bind:splitMode
      bind:splitRange
      bind:removePages
      bind:rotateDegrees
      bind:rotatePages
      bind:reorderOrder
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
