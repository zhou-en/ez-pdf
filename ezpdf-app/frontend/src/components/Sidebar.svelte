<script lang="ts">
  type Op = 'merge' | 'split' | 'remove' | 'rotate' | 'reorder';

  let {
    selectedOp = 'merge',
    onopSelected,
  }: { selectedOp?: Op; onopSelected?: (op: Op) => void } = $props();

  const ops: Op[] = ['merge', 'split', 'remove', 'rotate', 'reorder'];

  function select(op: Op) {
    onopSelected?.(op);
  }
</script>

<aside class="sidebar">
  {#each ops as op}
    <button class={op === selectedOp ? 'active' : ''} onclick={() => select(op)}>
      {op.charAt(0).toUpperCase() + op.slice(1)}
    </button>
  {/each}
</aside>

<style>
  .sidebar {
    width: 140px;
    background: #1e293b;
    display: flex;
    flex-direction: column;
    padding: 0.75rem 0;
  }

  button {
    background: none;
    border: none;
    color: #94a3b8;
    padding: 0.6rem 1rem;
    text-align: left;
    cursor: pointer;
    font-size: 0.875rem;
    border-left: 3px solid transparent;
  }

  button:hover {
    color: #e2e8f0;
    background: #334155;
  }

  button.active {
    color: #e2e8f0;
    border-left-color: #3b82f6;
    background: #334155;
  }
</style>
