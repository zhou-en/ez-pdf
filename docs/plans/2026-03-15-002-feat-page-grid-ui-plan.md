---
title: "feat: Page Grid UI — visual page tiles for Reorder, Remove, Rotate, Split"
type: feat
status: active
date: 2026-03-15
origin: docs/brainstorms/2026-03-15-page-grid-ui-brainstorm.md
---

# feat: Page Grid UI — Visual Page Tiles for Reorder, Remove, Rotate, Split

## Overview

Replace text-input–driven page selection in four operations with an interactive visual grid of
numbered page tiles. Each tile is a proportionally-sized CSS rectangle labeled with its page
number. Users drag tiles to reorder or click (with shift-click for ranges) to select pages for
removal, rotation, or splitting — then hit Run as before.

Merge is unchanged: it operates on files, not pages, and already has file-level drag-to-reorder.

## Problem Statement

The current text inputs (`removePages = "2,4-6"`, `reorderOrder = "3,1,2"`, etc.) require users
to know page numbers in advance and type comma-separated strings. This is error-prone and
unfriendly for anyone who hasn't memorised the document structure.

## Proposed Solution

1. When a file is dropped onto a grid op (reorder/remove/rotate/split), call a new `cmd_info`
   Tauri command to fetch page count and dimensions, then build a `pageTiles` array in App.svelte.
2. Render a `PageGrid.svelte` component in place of FileList for those four ops — a scrollable
   flex-wrap grid of proportional tiles.
3. On Run, derive the existing backend string arguments (`order`, `pages`, `range`) from the tile
   array state rather than text inputs.
4. Remove the now-redundant text input fields from OptionsPanel for these ops.

## Technical Approach

### New Tauri Command: `cmd_info`

`ezpdf_core::info()` already exists and returns `PdfInfo` (which `#[derive(Serialize)]`). It just
needs a thin Tauri wrapper:

```rust
// ezpdf-app/src/commands.rs
#[tauri::command]
pub fn cmd_info(input: String) -> Result<ezpdf_core::PdfInfo, String> {
    ezpdf_core::info(Path::new(&input)).map_err(|e| e.to_string())
}
```

Register in `lib.rs` `invoke_handler`. Export `PdfInfo` from `ezpdf-core/src/lib.rs`.

**TypeScript interface** (serde renders snake_case by default):

```typescript
// ezpdf-app/frontend/src/lib/tauri.ts
export interface PdfInfo {
  page_count: number;
  dimensions: [number, number][];   // [width_pts, height_pts] per page
}

export async function cmdInfo(input: string): Promise<PdfInfo> {
  return invoke<PdfInfo>('cmd_info', { input });
}
```

### PageTile Type and State in App.svelte

```typescript
// App.svelte
interface PageTile {
  pageNum: number;   // 1-based
  width: number;     // points, from PdfInfo.dimensions
  height: number;
  selected: boolean;
}

let pageTiles: PageTile[] = $state([]);
let splitOutputMode: 'combined' | 'individual' = $state('combined');
```

`pageTiles` is reset and re-populated whenever a file is added to reorder/remove/rotate/split.

### Loading Tiles in `addFiles()`

```typescript
// App.svelte — addFiles() extension
if (paths.length > 0 && ['reorder', 'remove', 'rotate', 'split'].includes(selectedOp)) {
  cmdInfo(paths[0]).then((info) => {
    pageTiles = info.dimensions.map(([w, h], i) => ({
      pageNum: i + 1,
      width: w,
      height: h,
      selected: false,
    }));
  }).catch(() => {});
}
```

### PageGrid.svelte Component

```
Props:
  tiles: PageTile[]
  mode: 'reorder' | 'select'
  ontiles: (updated: PageTile[]) => void

Internal state:
  draggingIndex: number | null   (reorder mode)
  lastSelectedIndex: number | null  (select mode, for shift-click)

Tile sizing:
  base width: 88px
  height: 88 * (tile.height / tile.width)   — aspect-ratio from PdfInfo
  min-height: 88px (square fallback for unusual dimensions)

Grid container:
  display: flex; flex-wrap: wrap; gap: 0.5rem
  max-height: 300px; overflow-y: auto   (see brainstorm: scrollable grid)

Selection visual: blue border + light blue background tint
Drag visual: opacity 0.4 on dragging tile (matches FileList.svelte pattern)
```

**Drag (reorder mode)** — same pattern as `FileList.svelte`:
- `draggable="true"` on each tile
- `ondragstart` sets `draggingIndex`
- `ondragover` calls `event.preventDefault()`
- `ondrop` splices tiles immutably, calls `ontiles(newTiles)`, resets index

**Click-select (select mode)** — with shift-click range (see brainstorm):
- Plain click: toggle `tile.selected`, update `lastSelectedIndex`
- Shift-click: set `selected = true` for all tiles between `lastSelectedIndex` and current index

### App.svelte Template

Replace `FileList` with conditional rendering:

```svelte
{#if files.length > 0}
  {#if selectedOp === 'merge'}
    <FileList {files} {pageCounts} onremove={removeFile} onreorder={reorderFiles} />
  {:else if ['reorder', 'remove', 'rotate', 'split'].includes(selectedOp)}
    <PageGrid
      tiles={pageTiles}
      mode={selectedOp === 'reorder' ? 'reorder' : 'select'}
      ontiles={(t) => (pageTiles = t)}
    />
  {/if}
  <div class="output-row">...</div>
{/if}
```

### Run Logic Derivations

```typescript
// App.svelte — run()
if (selectedOp === 'reorder') {
  const order = pageTiles.map((t) => t.pageNum).join(',');
  msg = await cmdReorder(files[0], order, out);

} else if (selectedOp === 'remove') {
  const pages = pageTiles.filter((t) => t.selected).map((t) => t.pageNum).join(',');
  msg = await cmdRemove(files[0], pages, out);

} else if (selectedOp === 'rotate') {
  const pages = pageTiles.filter((t) => t.selected).map((t) => t.pageNum).join(',') || null;
  msg = await cmdRotate(files[0], rotateDegrees, pages, out);

} else if (selectedOp === 'split') {
  const selected = pageTiles.filter((t) => t.selected).map((t) => t.pageNum);
  if (splitOutputMode === 'combined') {
    msg = await cmdSplitRange(files[0], selected.join(','), out);
  } else {
    // individual: one file per selected page
    const results: string[] = [];
    for (const p of selected) {
      const pageOut = out.replace(/\.pdf$/i, '') + `-page-${p}.pdf`;
      results.push(await cmdSplitRange(files[0], String(p), pageOut));
    }
    msg = results.join('\n');
  }
}
```

### OptionsPanel Changes

| Op | Before | After |
|----|--------|-------|
| Reorder | Text input (`reorderOrder`) | Empty — grid drives the order |
| Remove | Text input (`removePages`) | Empty — grid drives selection |
| Rotate | Degree select + pages text input | Degree select only (pages from grid) |
| Split | Range/Burst radio + range text | Output mode toggle (combined / individual) |

### Output Path for Split Individual Mode

When `splitOutputMode === 'individual'`, `handleSaveAs` should use `pickOutputDir` instead of
`saveOutputPath`, and the output path is used as a directory prefix.

Update `handleSaveAs`:
```typescript
const needsDir = (selectedOp === 'split' && splitOutputMode === 'individual') || selectedOp === 'extract';
```

### Validation Updates

```typescript
// run() validations
if (['remove', 'rotate', 'split'].includes(selectedOp)) {
  if (pageTiles.filter(t => t.selected).length === 0) {
    status = { type: 'error', message: 'Select at least one page.' };
    return;
  }
}
```

Remove old text-input validations for `removePages`, `reorderOrder`, `splitRange`.

## Implementation Phases

### Phase 24.1–24.2: Backend — `cmd_info`

**Tasks:**
- `24.1 [RED]` Add `cmd_info_returns_page_count_and_dimensions` test in `ezpdf-app/src/lib.rs`
  - Assert `page_count == 3` for `3page.pdf`
  - Assert `dimensions.len() == 3`
  - Assert each dimension `> 0.0`
- `24.2 [GREEN]` Add `cmd_info` to `commands.rs`; export `PdfInfo` from `ezpdf-core/src/lib.rs`;
  register in `lib.rs` invoke_handler; add `cmdInfo` wrapper + `PdfInfo` interface to `tauri.ts`

### Phase 24.3–24.4: PageGrid Component

**Tasks:**
- `24.3 [RED]` Write `PageGrid.test.ts`:
  - Renders correct number of tiles
  - Each tile shows its page number
  - Tiles have proportional aspect ratio (inline style)
  - Click toggles `selected` → calls `ontiles` with updated array
  - Shift-click selects range → calls `ontiles` with range selected
  - Drag-drop in reorder mode reorders tiles → calls `ontiles`
  - No reorder when dropping on same tile
  - Tile with `selected=true` has aria-pressed or data-selected attribute
- `24.4 [GREEN]` Create `PageGrid.svelte`

### Phase 24.5–24.6: App.svelte Integration

**Tasks:**
- `24.5 [RED]` Update `App.test.ts`:
  - Dropping file on remove op calls `cmdInfo` and builds pageTiles
  - Run on remove op calls `cmdRemove` with page numbers from selected tiles
  - Run on reorder op calls `cmdReorder` with tile order
  - Run on rotate op calls `cmdRotate` with selected tile page numbers
  - Run on split (combined) calls `cmdSplitRange` with selected pages
  - Run on split (individual) calls `cmdSplitRange` once per selected page
  - Error shown when no pages selected for remove/rotate/split
- `24.6 [GREEN]` Update `App.svelte`:
  - Add `pageTiles`, `splitOutputMode` state
  - Load tiles via `cmdInfo` in `addFiles()` for grid ops
  - Swap `FileList` for `PageGrid` in template for grid ops
  - Update `run()` with new derivation logic
  - Update `handleSaveAs` for split individual mode
  - Update validations

### Phase 24.7–24.8: OptionsPanel Cleanup

**Tasks:**
- `24.7 [RED]` Update `OptionsPanel.test.ts`:
  - Reorder op renders nothing (no text input)
  - Remove op renders nothing (no text input)
  - Rotate op renders degree selector only (no pages input)
  - Split op renders "Combined / Individual" toggle (no range/burst radio)
- `24.8 [GREEN]` Update `OptionsPanel.svelte`:
  - Remove `removePages`, `reorderOrder`, `splitRange`, `splitMode` bindings from split/remove/reorder panels
  - Rotate panel: remove pages text input
  - Split panel: replace range/burst radio with combined/individual toggle
  - Add `splitOutputMode` bindable prop

### Phase 24.9–24.10: Refactor & Review

- `24.9 [REFACTOR]` Remove now-unused state vars from App.svelte (`removePages`, `rotatePages`,
  `reorderOrder`, `splitRange`, `splitMode`); remove from OptionsPanel props; `cargo fmt`;
  `cargo clippy --workspace -- -D warnings`; `pnpm test` all green
- `24.10 [REVIEW]` Smoke test all 4 ops end-to-end with `tauri build`; verify all tests pass;
  verify validation errors; rebuild and confirm UI

## Alternative Approaches Considered

**Visual thumbnails (rejected):** Actual rendered page images would require `pdfium-render` crate
(~50MB binary, complex CI build for Linux). Not justified for a personal-use tool.
(see brainstorm: docs/brainstorms/2026-03-15-page-grid-ui-brainstorm.md)

**PageGrid in OptionsPanel (rejected):** Cleaner to render PageGrid directly in App.svelte
alongside FileList — keeps OptionsPanel as a lightweight options form and avoids deep prop-drilling
of `pageTiles` state.

## Acceptance Criteria

### Functional

- [ ] Dropping a PDF on Reorder/Remove/Rotate/Split shows a proportional-tile page grid
- [ ] Reorder: dragging tiles reorders them; Run produces a reordered PDF
- [ ] Remove: clicking tiles toggles selection; Run removes selected pages
- [ ] Rotate: clicking tiles selects them; global degree selector applies to all selected; Run rotates
- [ ] Split (combined): selected pages extracted into one output PDF
- [ ] Split (individual): each selected page becomes a separate PDF file
- [ ] Shift-click selects all tiles between last-clicked and current tile
- [ ] Grid scrolls vertically at ≥300px height; UI below remains visible
- [ ] Validation error if Run clicked with no pages selected (remove/rotate/split)
- [ ] Merge unchanged — file-level drag-to-reorder still works

### Non-Functional

- [ ] No new Rust or npm dependencies added
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `pnpm test` — all frontend tests pass
- [ ] `cargo test -p ezpdf-app` — all backend tests pass
- [ ] `tauri build` succeeds

## Dependencies & Risks

- `cmd_info` wraps existing `ezpdf_core::info()` — no new core logic needed
- `PdfInfo` already `#[derive(Serialize)]` — zero Rust changes to core
- Split individual mode calls `cmdSplitRange` N times synchronously in `run()` — acceptable for personal use (no parallelism needed)
- Removing `splitMode` / `removePages` / `rotatePages` / `reorderOrder` from state: ensure no dangling `bind:` references in App.svelte template

## Files Changed

| File | Change |
|------|--------|
| `ezpdf-core/src/lib.rs` | Export `PdfInfo` |
| `ezpdf-app/src/commands.rs` | Add `cmd_info` |
| `ezpdf-app/src/lib.rs` | Register `cmd_info`; add test |
| `ezpdf-app/frontend/src/lib/tauri.ts` | Add `PdfInfo` interface + `cmdInfo` wrapper |
| `ezpdf-app/frontend/src/App.svelte` | Add pageTiles state; load via cmdInfo; swap FileList→PageGrid; update run() |
| `ezpdf-app/frontend/src/components/PageGrid.svelte` | **New** |
| `ezpdf-app/frontend/src/components/PageGrid.test.ts` | **New** |
| `ezpdf-app/frontend/src/components/OptionsPanel.svelte` | Remove text inputs; add split mode toggle |
| `ezpdf-app/frontend/src/App.test.ts` | Update for new grid-driven behavior |
| `ezpdf-app/frontend/src/components/OptionsPanel.test.ts` | Update panel assertions |

## Sources & References

### Origin

- **Brainstorm:** [docs/brainstorms/2026-03-15-page-grid-ui-brainstorm.md](../brainstorms/2026-03-15-page-grid-ui-brainstorm.md)
  Key decisions carried forward: numbered tiles (not thumbnails); scrollable 300px grid;
  shift-click range selection; split combined/individual toggle; no drag ghost placeholder.

### Internal References

- `ezpdf_core::info()` — `ezpdf-core/src/info.rs:26`
- `PdfInfo` struct — `ezpdf-core/src/info.rs:9`
- FileList drag pattern — `ezpdf-app/frontend/src/components/FileList.svelte`
- Tauri command pattern — `ezpdf-app/src/commands.rs`
- App.svelte state pattern — `ezpdf-app/frontend/src/App.svelte`
- Existing test patterns — `ezpdf-app/frontend/src/App.test.ts`
