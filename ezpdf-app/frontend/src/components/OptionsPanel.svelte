<script lang="ts">
  type Op = 'merge' | 'split' | 'remove' | 'rotate' | 'reorder';
  type SplitMode = 'range' | 'burst';

  let {
    op,
    splitMode = $bindable<SplitMode>('range'),
    splitRange = $bindable(''),
    removePages = $bindable(''),
    rotateDegrees = $bindable(90),
    rotatePages = $bindable(''),
    reorderOrder = $bindable(''),
  }: {
    op: Op;
    splitMode?: SplitMode;
    splitRange?: string;
    removePages?: string;
    rotateDegrees?: number;
    rotatePages?: string;
    reorderOrder?: string;
  } = $props();
</script>

{#if op === 'merge'}
  <!-- no op-specific inputs for merge -->
{:else if op === 'split'}
  <div class="options">
    <div class="radio-row">
      <label>
        <input type="radio" bind:group={splitMode} value="range" />
        Extract range
      </label>
      <label>
        <input type="radio" bind:group={splitMode} value="burst" />
        Burst all pages
      </label>
    </div>
    {#if splitMode === 'range'}
      <label class="field">
        <span>Range</span>
        <input
          type="text"
          bind:value={splitRange}
          aria-label="Range"
          placeholder="e.g. 1-3"
        />
      </label>
    {/if}
  </div>
{:else if op === 'remove'}
  <div class="options">
    <label class="field">
      <span>Pages to remove</span>
      <input
        type="text"
        bind:value={removePages}
        aria-label="Pages"
        placeholder="e.g. 2,4-6"
      />
    </label>
  </div>
{:else if op === 'rotate'}
  <div class="options">
    <label class="field">
      <span>Degrees</span>
      <select bind:value={rotateDegrees} aria-label="Degrees">
        <option value={90}>90°</option>
        <option value={180}>180°</option>
        <option value={270}>270°</option>
        <option value={-90}>-90°</option>
      </select>
    </label>
    <label class="field">
      <span>Pages (optional)</span>
      <input
        type="text"
        bind:value={rotatePages}
        aria-label="Pages"
        placeholder="all"
      />
    </label>
  </div>
{:else if op === 'reorder'}
  <div class="options">
    <label class="field">
      <span>New page order</span>
      <input
        type="text"
        bind:value={reorderOrder}
        aria-label="Order"
        placeholder="e.g. 3,1,2"
      />
    </label>
  </div>
{/if}

<style>
  .options {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
  }

  .radio-row {
    display: flex;
    gap: 1.5rem;
  }

  .radio-row label {
    display: flex;
    align-items: center;
    gap: 0.4rem;
    cursor: pointer;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
  }

  .field span {
    font-size: 0.8rem;
    font-weight: 500;
    color: #475569;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .field input,
  .field select {
    padding: 0.4rem 0.6rem;
    border: 1px solid #cbd5e1;
    border-radius: 5px;
    font-size: 0.9rem;
    width: 220px;
    background: white;
  }

  .field input:focus,
  .field select:focus {
    outline: 2px solid #3b82f6;
    outline-offset: 1px;
    border-color: transparent;
  }
</style>
