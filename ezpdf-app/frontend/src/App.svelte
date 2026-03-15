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
    if (files.length === 0) return;
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

    <button onclick={run} disabled={files.length === 0 || running}>
      Run {selectedOp.charAt(0).toUpperCase() + selectedOp.slice(1)}
    </button>

    {#if status.type !== 'idle'}
      <p class="status {status.type}">{status.message}</p>
    {/if}
  </main>
</div>
