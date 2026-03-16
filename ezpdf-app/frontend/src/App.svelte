<script lang="ts">
  import Sidebar from './components/Sidebar.svelte';
  import DropZone from './components/DropZone.svelte';
  import OptionsPanel from './components/OptionsPanel.svelte';
  import { cmdMerge, cmdSplitRange, cmdSplitEach, cmdRemove, cmdRotate, cmdReorder, cmdPageCount } from './lib/tauri';

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
    // Burst mode outputs a directory of pages, not a single PDF
    if (op === 'split' && splitMode === 'burst') {
      return `${dir}${base}-pages`;
    }
    return `${dir}${base}-${opSuffix[op]}.pdf`;
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
      const out = defaultOutput(selectedOp);
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
  <Sidebar {selectedOp} onopSelected={(op) => { selectedOp = op; status = { type: 'idle', message: '' }; }} />

  <main>
    <DropZone onfilesAdded={addFiles} />

    {#if files.length > 0}
      <ul class="file-list">
        {#each files as file, i}
          <li>
            <span>{basename(file)}</span>
            {#if pageCounts[file] !== undefined}
              <span class="page-count">{pageCounts[file]} {pageCounts[file] === 1 ? 'page' : 'pages'}</span>
            {/if}
            <button
              class="remove-btn"
              aria-label="Remove {basename(file)}"
              onclick={() => removeFile(i)}
            >×</button>
          </li>
        {/each}
      </ul>
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

    {#if status.type !== 'idle'}
      <p class="status {status.type}">{status.message}</p>
    {/if}
  </main>
</div>

<style>
  .app {
    display: flex;
    height: 100vh;
    font-family: system-ui, sans-serif;
  }

  main {
    flex: 1;
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .file-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  .file-list li {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.25rem 0.5rem;
    background: #f3f4f6;
    border-radius: 4px;
    margin-bottom: 0.25rem;
    font-size: 0.875rem;
  }

  .page-count {
    margin-left: auto;
    margin-right: 0.5rem;
    font-size: 0.75rem;
    color: #6b7280;
  }

  .remove-btn {
    background: none;
    border: none;
    color: #9ca3af;
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0 0.25rem;
    border-radius: 3px;
  }

  .remove-btn:hover {
    color: #ef4444;
    background: #fee2e2;
  }

  .run-btn {
    align-self: flex-start;
    padding: 0.5rem 1.5rem;
    background: #2563eb;
    color: white;
    border: none;
    border-radius: 6px;
    font-size: 1rem;
    cursor: pointer;
  }

  .run-btn:disabled {
    background: #93c5fd;
    cursor: not-allowed;
  }

  .run-btn:not(:disabled):hover {
    background: #1d4ed8;
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
