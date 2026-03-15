<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { onFileDrop } from '../lib/dnd';

  let { onfilesAdded }: { onfilesAdded?: (paths: string[]) => void } = $props();

  let files: string[] = $state([]);
  let unlisten: (() => void) | undefined;

  function basename(path: string): string {
    return path.replace(/^.*[/\\]/, '');
  }

  onMount(async () => {
    unlisten = await onFileDrop((paths) => {
      const pdfs = paths.filter((p) => p.toLowerCase().endsWith('.pdf'));
      if (pdfs.length > 0) {
        files = [...files, ...pdfs];
        onfilesAdded?.(pdfs);
      }
    });
  });

  onDestroy(() => {
    unlisten?.();
  });
</script>

<div class="drop-zone">
  <p>Drop PDF files here</p>
  {#if files.length > 0}
    <ul>
      {#each files as file}
        <li>{basename(file)}</li>
      {/each}
    </ul>
  {/if}
</div>

<style>
  .drop-zone {
    border: 2px dashed #475569;
    border-radius: 8px;
    padding: 2rem;
    text-align: center;
    color: #64748b;
    transition: border-color 0.15s, background 0.15s;
  }

  .drop-zone:hover {
    border-color: #3b82f6;
    background: #f0f7ff;
  }

  p {
    margin: 0 0 0.5rem;
    font-size: 0.9rem;
  }

  ul {
    list-style: none;
    padding: 0;
    margin: 0.5rem 0 0;
    text-align: left;
  }

  li {
    font-size: 0.8rem;
    padding: 0.2rem 0.4rem;
    background: #e2e8f0;
    border-radius: 3px;
    margin-bottom: 0.2rem;
    color: #334155;
  }
</style>
