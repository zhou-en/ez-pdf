<script lang="ts">
  type Op = 'merge' | 'split' | 'remove' | 'rotate' | 'reorder';
  type SplitMode = 'range' | 'burst';

  let { op = 'merge' }: { op: Op } = $props();

  let splitMode: SplitMode = $state('range');
</script>

{#if op === 'merge'}
  <!-- no op-specific inputs for merge -->
{:else if op === 'split'}
  <div>
    <label>
      <input type="radio" bind:group={splitMode} value="range" />
      Extract range
    </label>
    <label>
      <input type="radio" bind:group={splitMode} value="burst" />
      Burst all pages
    </label>
    {#if splitMode === 'range'}
      <label>
        Range
        <input type="text" aria-label="Range" placeholder="e.g. 1-3" />
      </label>
    {/if}
  </div>
{:else if op === 'remove'}
  <div>
    <label>
      Pages
      <input type="text" aria-label="Pages" placeholder="e.g. 2,4-6" />
    </label>
  </div>
{:else if op === 'rotate'}
  <div>
    <label>
      Degrees
      <select aria-label="Degrees">
        <option value="90">90°</option>
        <option value="180">180°</option>
        <option value="270">270°</option>
        <option value="-90">-90°</option>
      </select>
    </label>
    <label>
      Pages
      <input type="text" aria-label="Pages" placeholder="all" />
    </label>
  </div>
{:else if op === 'reorder'}
  <div>
    <label>
      Order
      <input type="text" aria-label="Order" placeholder="e.g. 3,1,2" />
    </label>
  </div>
{/if}
