<script lang="ts">
  import type { PdfMetadata, Bookmark } from '../lib/tauri';

  type Op = 'merge' | 'split' | 'remove' | 'rotate' | 'reorder' | 'metadata' | 'watermark' | 'bookmarks' | 'extract';
  type SplitMode = 'range' | 'burst';

  let {
    op,
    splitMode = $bindable<SplitMode>('range'),
    splitRange = $bindable(''),
    removePages = $bindable(''),
    rotateDegrees = $bindable(90),
    rotatePages = $bindable(''),
    reorderOrder = $bindable(''),
    metaTitle = $bindable(''),
    metaAuthor = $bindable(''),
    metaSubject = $bindable(''),
    metaKeywords = $bindable(''),
    loadedMeta = $bindable<PdfMetadata | null>(null),
    watermarkText = $bindable(''),
    watermarkFontSize = $bindable(48),
    watermarkOpacity = $bindable(0.3),
    watermarkPages = $bindable(''),
    bookmarksList = $bindable<Bookmark[]>([]),
    bookmarkTitle = $bindable(''),
    bookmarkPage = $bindable(1),
  }: {
    op: Op;
    splitMode?: SplitMode;
    splitRange?: string;
    removePages?: string;
    rotateDegrees?: number;
    rotatePages?: string;
    reorderOrder?: string;
    metaTitle?: string;
    metaAuthor?: string;
    metaSubject?: string;
    metaKeywords?: string;
    loadedMeta?: PdfMetadata | null;
    watermarkText?: string;
    watermarkFontSize?: number;
    watermarkOpacity?: number;
    watermarkPages?: string;
    bookmarksList?: Bookmark[];
    bookmarkTitle?: string;
    bookmarkPage?: number;
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
{:else if op === 'metadata'}
  <div class="options">
    {#if loadedMeta}
      <p class="hint">Loaded existing metadata — edit fields below.</p>
    {/if}
    <label class="field">
      <span>Title</span>
      <input type="text" bind:value={metaTitle} aria-label="Title" placeholder="Document title" />
    </label>
    <label class="field">
      <span>Author</span>
      <input type="text" bind:value={metaAuthor} aria-label="Author" placeholder="Author name" />
    </label>
    <label class="field">
      <span>Subject</span>
      <input type="text" bind:value={metaSubject} aria-label="Subject" placeholder="Subject" />
    </label>
    <label class="field">
      <span>Keywords</span>
      <input type="text" bind:value={metaKeywords} aria-label="Keywords" placeholder="Comma-separated keywords" />
    </label>
  </div>
{:else if op === 'watermark'}
  <div class="options">
    <label class="field">
      <span>Watermark text</span>
      <input type="text" bind:value={watermarkText} aria-label="Watermark text" placeholder="e.g. CONFIDENTIAL" />
    </label>
    <label class="field">
      <span>Font size (pt)</span>
      <input type="number" bind:value={watermarkFontSize} aria-label="Font size" min="8" max="200" />
    </label>
    <label class="field">
      <span>Opacity (0–1)</span>
      <input type="number" bind:value={watermarkOpacity} aria-label="Opacity" min="0" max="1" step="0.05" />
    </label>
    <label class="field">
      <span>Pages (optional)</span>
      <input type="text" bind:value={watermarkPages} aria-label="Pages" placeholder="all" />
    </label>
  </div>
{:else if op === 'bookmarks'}
  <div class="options">
    {#if bookmarksList.length > 0}
      <div class="bookmark-list" aria-label="Bookmarks list">
        {#each bookmarksList as bm}
          <div class="bookmark-item">
            <span class="bm-title">{bm.title}</span>
            <span class="bm-page">p.{bm.page}</span>
          </div>
        {/each}
      </div>
    {:else}
      <p class="hint">No bookmarks — add one below.</p>
    {/if}
    <label class="field">
      <span>Bookmark title</span>
      <input type="text" bind:value={bookmarkTitle} aria-label="Bookmark title" placeholder="Chapter 1" />
    </label>
    <label class="field">
      <span>Page</span>
      <input type="number" bind:value={bookmarkPage} aria-label="Page" min="1" />
    </label>
  </div>
{:else if op === 'extract'}
  <div class="options">
    <p class="hint">Images will be saved to the selected output folder.</p>
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

  .hint {
    font-size: 0.8rem;
    color: #64748b;
    margin: 0;
  }

  .bookmark-list {
    display: flex;
    flex-direction: column;
    gap: 0.3rem;
    max-height: 140px;
    overflow-y: auto;
    border: 1px solid #cbd5e1;
    border-radius: 5px;
    padding: 0.4rem 0.6rem;
    background: #f8fafc;
  }

  .bookmark-item {
    display: flex;
    gap: 0.75rem;
    font-size: 0.875rem;
  }

  .bm-title {
    flex: 1;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .bm-page {
    color: #64748b;
    white-space: nowrap;
  }
</style>
