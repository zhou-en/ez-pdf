<script lang="ts">
  let {
    files,
    pageCounts = {},
    onremove,
    onreorder,
  }: {
    files: string[];
    pageCounts?: Record<string, number>;
    onremove: (index: number) => void;
    onreorder: (from: number, to: number) => void;
  } = $props();

  let draggingIndex: number | null = $state(null);
  let isDragging = $state(false);

  function basename(path: string): string {
    return path.replace(/^.*[/\\]/, '');
  }

  function handlePointerDown(index: number) {
    draggingIndex = index;
    isDragging = false;
  }

  function handlePointerMove() {
    if (draggingIndex !== null) isDragging = true;
  }

  function handlePointerUp(index: number) {
    if (isDragging && draggingIndex !== null && draggingIndex !== index) {
      onreorder(draggingIndex, index);
    }
    draggingIndex = null;
    isDragging = false;
  }

  function handlePointerCancel() {
    draggingIndex = null;
    isDragging = false;
  }
</script>

<ul class="file-list" onpointermove={handlePointerMove}>
  {#each files as file, i}
    <li
      role="listitem"
      class:dragging={draggingIndex === i && isDragging}
      onpointerdown={() => handlePointerDown(i)}
      onpointerup={() => handlePointerUp(i)}
      onpointercancel={handlePointerCancel}
    >
      <span class="name">{basename(file)}</span>
      {#if pageCounts[file] !== undefined}
        <span class="page-count">
          {pageCounts[file]} {pageCounts[file] === 1 ? 'page' : 'pages'}
        </span>
      {/if}
      <button
        class="remove-btn"
        aria-label="Remove {basename(file)}"
        onpointerdown={(e) => e.stopPropagation()}
        onclick={() => onremove(i)}
      >×</button>
    </li>
  {/each}
</ul>

<style>
  .file-list {
    list-style: none;
    padding: 0;
    margin: 0;
  }

  li {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.25rem 0.5rem;
    background: var(--file-item-bg, #f3f4f6);
    border-radius: 4px;
    margin-bottom: 0.25rem;
    font-size: 0.875rem;
    cursor: grab;
    user-select: none;
  }

  li.dragging {
    opacity: 0.4;
  }

  li:active {
    cursor: grabbing;
  }

  .name {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .page-count {
    font-size: 0.75rem;
    color: var(--text-muted, #6b7280);
    white-space: nowrap;
  }

  .remove-btn {
    background: none;
    border: none;
    color: var(--text-muted, #9ca3af);
    cursor: pointer;
    font-size: 1rem;
    line-height: 1;
    padding: 0 0.25rem;
    border-radius: 3px;
    flex-shrink: 0;
  }

  .remove-btn:hover {
    color: #ef4444;
    background: #fee2e2;
  }
</style>
