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
