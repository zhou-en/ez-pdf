<script lang="ts">
  import Sidebar from './components/Sidebar.svelte';
  import DropZone from './components/DropZone.svelte';
  import OptionsPanel from './components/OptionsPanel.svelte';
  import { cmdMerge, cmdSplitRange, cmdSplitEach, cmdRemove, cmdRotate, cmdReorder } from './lib/tauri';

  type Op = 'merge' | 'split' | 'remove' | 'rotate' | 'reorder';
  type Status = { type: 'idle' | 'success' | 'error'; message: string };

  let files: string[] = $state([]);
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
    return `${dir}${base}-${opSuffix[op]}.pdf`;
  }

  async function run() {
    if (files.length === 0) {
      status = { type: 'error', message: 'Add at least one PDF file.' };
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
  <Sidebar {selectedOp} onopSelected={(op) => (selectedOp = op)} />

  <main>
    <DropZone onfilesAdded={(paths) => { files = [...files, ...paths]; }} />

    {#if files.length > 0}
      <ul class="file-list">
        {#each files as file}
          <li>{basename(file)}</li>
        {/each}
      </ul>
    {/if}

    <OptionsPanel op={selectedOp} />

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
    padding: 0.25rem 0.5rem;
    background: #f3f4f6;
    border-radius: 4px;
    margin-bottom: 0.25rem;
    font-size: 0.875rem;
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
