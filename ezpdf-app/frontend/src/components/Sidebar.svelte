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
