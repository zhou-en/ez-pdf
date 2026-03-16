<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { onFileDrop } from '../lib/dnd';
  import { openPdfFiles } from '../lib/dialog';

  let { onfilesAdded }: { onfilesAdded?: (paths: string[]) => void } = $props();

  let unlisten: (() => void) | undefined;

  onMount(async () => {
    unlisten = await onFileDrop((paths) => {
      const pdfs = paths.filter((p) => p.toLowerCase().endsWith('.pdf'));
      if (pdfs.length > 0) {
        onfilesAdded?.(pdfs);
      }
    });
  });

  onDestroy(() => {
    unlisten?.();
  });

  async function browse() {
    const files = await openPdfFiles();
    if (files && files.length > 0) {
      onfilesAdded?.(files);
    }
  }
</script>

<button class="drop-zone" onclick={browse} aria-label="Browse for PDF files">
  <p>Drop PDF files here</p>
  <span class="hint">or click to browse</span>
</button>

<style>
  .drop-zone {
    width: 100%;
    border: 2px dashed #475569;
    border-radius: 8px;
    padding: 2rem;
    text-align: center;
    color: #64748b;
    background: transparent;
    cursor: pointer;
    transition: border-color 0.15s, background 0.15s;
    font-family: inherit;
  }

  .drop-zone:hover {
    border-color: #3b82f6;
    background: #f0f7ff;
    color: #3b82f6;
  }

  p {
    margin: 0 0 0.25rem;
    font-size: 0.9rem;
  }

  .hint {
    font-size: 0.78rem;
    opacity: 0.7;
  }
</style>
