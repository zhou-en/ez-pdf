---
date: 2026-03-15
topic: page-grid-ui
---

# Page Grid UI — Visual Page Selection for Reorder, Remove, Rotate, Split

## What We're Building

Replace the text-input–driven page selection in four operations (Reorder, Remove, Rotate, Split)
with a visual page grid: a row of numbered tiles, one per PDF page, that the user can interact
with directly before clicking Run.

Each tile is a proportionally-sized rectangle (aspect ratio derived from the page's real dimensions)
labeled with its page number. No PDF renderer is needed — tiles are pure CSS, sized using the
`dimensions` array already available from `ezpdf_core::info()`.

**Per-operation behaviour:**

| Op | Interaction | What drives the backend call |
|----|-------------|------------------------------|
| Reorder | Drag tiles into new order | Array index order → `"3,1,2"` string |
| Remove | Click to select (toggle); selected = marked for deletion | Selected page numbers → `"2,4-6"` |
| Rotate | Click to select; selected share a global rotation setting | Selected page numbers → `pages` arg |
| Split | Click to select pages to extract | Selected page numbers → range string |
| Merge | Unchanged — file-level drag reorder (already works) | File array order |

## Why This Approach

**Option A (chosen) — Numbered tiles:** Simple CSS boxes with page number + proportional sizing.
Fast to implement, zero new dependencies, works offline, consistent with the existing app aesthetic.

**Option B (rejected) — Visual thumbnails:** Actual page images. Would require `pdfium-render`
(~50 MB dependency, complex CI build). Far more complex for marginal benefit on a personal-use tool.

## Key Decisions

- **Proportional sizing**: Call the new `cmd_info` Tauri command (wrapping `ezpdf_core::info()`)
  when a file is dropped. Store `dimensions: [width, height][]` per file. Each tile's CSS aspect
  ratio = `width / height`. Tile base width ≈ 90px so a full grid fits without scrolling for
  typical documents.

- **Shared `PageGrid` component**: One component handles all four ops. Props control the mode:
  `mode: 'reorder' | 'remove' | 'rotate' | 'split'`. Reorder mode enables drag handles; the
  other three modes enable click-to-select.

- **Rotate: global setting, per-selection**: All selected tiles share one rotation value (the
  existing `rotateDegrees` state). No per-tile rotation UI — keeps the component simple. Users
  wanting different rotations per page can run Rotate multiple times.

- **Merge stays as-is**: Merge operates on files, not pages. The existing file-level
  drag-to-reorder in FileList already solves this.

- **Remove the text inputs**: Once the grid is in place, the manual text inputs for
  `removePages`, `rotatePages`, `reorderOrder`, and `splitRange` (range mode) are replaced by
  the grid selection. The "Extract range" / "Burst" split mode radio is also replaced — the grid
  selection IS the range; Burst becomes a "select all" shortcut.

- **New Tauri command**: `cmd_info(input: String) → Result<PdfInfo, String>` exposing
  `page_count` and `dimensions` to the frontend. `PdfInfo` already derives `Serialize`.

## Component Sketch

```
PageGrid.svelte
  props: { pages: PageTile[], mode, onreorder, onselect }

PageTile = { pageNum: number, width: number, height: number, selected: boolean }

Layout: CSS flex-wrap row, tile width ~90px, height = 90 * (h/w)
Selected state: blue border + light blue background
Drag (reorder mode): HTML5 draggable, same pattern as FileList.svelte
```

## Open Questions

_(none — resolved during brainstorming)_

## Resolved Questions

- **Thumbnails or tiles?** → Tiles (numbered boxes, proportional aspect ratio). No PDF renderer.
- **Per-tile rotation?** → No. Global rotation setting applies to all selected tiles.
- **Merge changes?** → None. File-level reorder already works.
- **Split: range vs. selection?** → Grid selection replaces text range input. Burst = select all.
- **Large PDFs?** → Scrollable grid with fixed max-height (~300px). Rest of UI stays visible below.
- **Multi-select behaviour?** → Click to toggle individual tiles; shift-click to select a range.
- **Split output?** → Toggle between two modes: (A) selected pages → one combined PDF, (B) each selected page → its own PDF.
- **Reorder drag placeholder?** → No ghost slot. Tiles shift on hover, consistent with existing file list.
