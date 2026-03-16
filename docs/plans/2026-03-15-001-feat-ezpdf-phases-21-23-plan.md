---
title: "feat: ezpdf Phases 21–23 — UX Polish, CLI Feature UI, Distribution"
type: feat
status: active
date: 2026-03-15
---

# ezpdf Phases 21–23: UX Polish, CLI Feature UI, Distribution

## Overview

Three follow-on phases to the completed Phase 20 desktop app (Tauri v2 + Svelte 5):

| Phase | Name | Target |
|-------|------|--------|
| 21 | UX Polish | v2.1 |
| 22 | CLI Features in App UI | v2.2 |
| 23 | Distribution & Packaging | v2.3 |

Each phase follows the established `[RED] → [GREEN] → [REFACTOR] → [REVIEW]` TDD workflow used in Phases 1–20.

---

## Phase 21: UX Polish

**Target:** v2.1

### Features

1. **Drag-to-reorder merge file list** — users can reorder files in the Merge file list via drag-and-drop before running
2. **Output path picker** — "Save As…" button lets users override the auto-generated output path using a native file/folder dialog
3. **Dark mode** — respects `prefers-color-scheme: dark`; CSS custom properties switch the full UI
4. **Progress indicator** — spinner/indeterminate bar while a long operation is running (replaces the static "Running…" label)

### Technical Approach

#### 21.1 — Drag-to-reorder

- Use the HTML5 Drag-and-Drop API (`draggable`, `ondragstart`, `ondragover`, `ondrop`) on file list `<li>` items in `App.svelte`
- Track `draggingIndex: number | null` as `$state`
- On drop: splice `filesByOp[selectedOp]` to move the dragged item — use immutable spread pattern
- Scope to Merge op only (split/remove/rotate/reorder each take a single file)
- Extract file list into a real `FileList.svelte` component (currently a stub at `ezpdf-app/frontend/src/components/FileList.svelte`)

```svelte
<!-- ezpdf-app/frontend/src/components/FileList.svelte -->
<script lang="ts">
  let { files, pageCounts, onremove, onreorder }: {
    files: string[];
    pageCounts: Record<string, number>;
    onremove: (i: number) => void;
    onreorder: (from: number, to: number) => void;
  } = $props();

  let draggingIndex: number | null = $state(null);
</script>
```

#### 21.2 — Output path picker

- Add `outputOverride: Record<Op, string>` state to `App.svelte` (mirrors `filesByOp` shape)
- "Save As…" button calls `save({ defaultPath, filters })` from `@tauri-apps/plugin-dialog` (already installed)
- Burst mode uses `open({ directory: true })` instead of save dialog
- `defaultOutput()` falls back to auto-path when `outputOverride[selectedOp]` is empty
- Add `dialog:allow-save` to `capabilities/default.json` (already present)
- Update `lib/dialog.ts` with `saveOutputPath(op, defaultPath)` and `pickOutputDir()` helpers

```typescript
// ezpdf-app/frontend/src/lib/dialog.ts
export async function saveOutputPath(defaultPath: string): Promise<string | null>
export async function pickOutputDir(defaultPath: string): Promise<string | null>
```

#### 21.3 — Dark mode

- Define CSS custom properties on `:root` (light values) and override inside `@media (prefers-color-scheme: dark)`
- Properties: `--bg`, `--bg-sidebar`, `--text`, `--text-muted`, `--border`, `--run-btn`, `--status-success-bg`, `--status-error-bg`, etc.
- Apply to `App.svelte`, `Sidebar.svelte`, `DropZone.svelte`, `OptionsPanel.svelte`
- No JS required — pure CSS media query

```css
/* App.svelte <style> */
:root {
  --bg: #ffffff;
  --bg-sidebar: #1e293b;
  --text: #111827;
  --text-muted: #6b7280;
  --border: #e5e7eb;
}
@media (prefers-color-scheme: dark) {
  :root {
    --bg: #111827;
    --bg-sidebar: #0f172a;
    --text: #f9fafb;
    --text-muted: #9ca3af;
    --border: #374151;
  }
}
```

#### 21.4 — Progress indicator

- `running` state already exists in `App.svelte`
- Replace "Running…" text with an animated CSS spinner component
- Add `ProgressBar.svelte` — indeterminate bar (CSS animation, no Rust backend changes needed)
- Show below run button while `running === true`

```svelte
<!-- ezpdf-app/frontend/src/components/ProgressBar.svelte -->
<script lang="ts">
  let { visible }: { visible: boolean } = $props();
</script>
{#if visible}
  <div class="progress-bar"><div class="progress-bar__fill"></div></div>
{/if}
```

### Tasks

```
- [ ] 21.1 [RED]     FileList.svelte tests: drag start/over/drop reorders items; remove still works
- [ ] 21.2 [GREEN]   Extract FileList.svelte from App.svelte stub; wire ondragstart/ondragover/ondrop; call onreorder callback
- [ ] 21.3 [RED]     App.test.ts: output override — after clicking Save As, Run uses override path not default
- [ ] 21.4 [GREEN]   Add saveOutputPath() + pickOutputDir() to dialog.ts; add outputOverride state; wire Save As button
- [ ] 21.5 [RED]     Dark mode snapshot/visual test OR manual checklist in review
- [ ] 21.6 [GREEN]   Add CSS custom properties to all component <style> blocks; add @media dark override
- [ ] 21.7 [RED]     ProgressBar.test.ts: renders when visible=true, hidden when false
- [ ] 21.8 [GREEN]   Add ProgressBar.svelte; render below run button in App.svelte when running===true
- [ ] 21.9 [REFACTOR] Consolidate CSS variables; ensure no hardcoded color values remain; cargo fmt + clippy
- [ ] 21.10 [REVIEW]  All frontend tests pass; dark mode looks correct on macOS; Save As path flows to backend; rebuild smoke test
```

### Definition of Done

- [ ] Merge file list is drag-reorderable (within Merge op only)
- [ ] Save As button appears for each op; selected path is used in the backend call
- [ ] App respects `prefers-color-scheme: dark` — all surfaces switch
- [ ] Progress indicator visible while operation runs
- [ ] All existing 37 frontend tests still pass + new tests added
- [ ] `cargo clippy --workspace -- -D warnings` clean

---

## Phase 22: CLI Features in App UI

**Target:** v2.2

### Features

Add four operations that already exist in `ezpdf-core` (and the CLI) but have no desktop UI:

| Op | Core Function | CLI Command |
|----|--------------|-------------|
| Metadata editor | `get_metadata`, `set_metadata` | `ezpdf meta` |
| Watermark | `watermark` | `ezpdf watermark` |
| Bookmarks | `list_bookmarks`, `add_bookmark` | `ezpdf bookmarks` |
| Image extraction | `extract_images` | `ezpdf images` |

### Technical Approach

#### Architecture

Each new op follows the same wire-up pattern:
1. Add `#[tauri::command] pub fn cmd_<op>(...)` to `ezpdf-app/src/commands.rs`
2. Register in `tauri::generate_handler![]` in `lib.rs`
3. Add typed wrapper in `frontend/src/lib/tauri.ts`
4. Add op to `Op` type in `App.svelte` (currently `'merge' | 'split' | 'remove' | 'rotate' | 'reorder'`)
5. Add op-specific form fields to `OptionsPanel.svelte`
6. Add op handler branch in `run()` in `App.svelte`

#### 22.1 — Metadata Editor

- Two-phase UI: (1) on file load, fetch metadata and display current values; (2) allow editing and write on Run
- New Tauri commands: `cmd_get_metadata(input) → Result<PdfMetadata, String>` and `cmd_set_metadata(input, updates, output) → Result<String, String>`
- `PdfMetadata` struct must be serializable — add `#[derive(serde::Serialize)]` to `ezpdf_core::metadata::PdfMetadata`
- OptionsPanel shows labeled text inputs for: Title, Author, Subject, Keywords, Creator

```rust
// ezpdf-app/src/commands.rs
#[tauri::command]
pub fn cmd_get_metadata(input: String) -> Result<serde_json::Value, String>

#[tauri::command]
pub fn cmd_set_metadata(
    input: String,
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    keywords: Option<String>,
    output: String,
) -> Result<String, String>
```

#### 22.2 — Watermark

- OptionsPanel fields: watermark text (string), font size (number, default 48), opacity (0–1 slider, default 0.3), angle (degrees, default 45)
- New command: `cmd_watermark(input, text, font_size, opacity, angle_deg, output) → Result<String, String>`

#### 22.3 — Bookmarks

- Two sub-modes: List (read-only display) and Add (append a bookmark)
- List: `cmd_list_bookmarks(input) → Result<Vec<Bookmark>, String>` — display in a read-only `<pre>` or table
- Add: `cmd_add_bookmark(input, title, page, output) → Result<String, String>`
- `Bookmark` struct needs `#[derive(serde::Serialize)]`

#### 22.4 — Image Extraction

- Output is a directory (use `pickOutputDir()` from Phase 21)
- New command: `cmd_extract_images(input, output_dir) → Result<String, String>`
- Simpler than other ops: no extra options needed, just input file + output directory

### Tasks

```
- [ ] 22.1  [RED]     cmd_get_metadata + cmd_set_metadata tests in ezpdf-app/src/lib.rs
- [ ] 22.2  [GREEN]   Implement commands; add serde::Serialize to PdfMetadata; register in invoke_handler
- [ ] 22.3  [RED]     App.test.ts: metadata op appears in sidebar; fields render; Run calls cmd_set_metadata
- [ ] 22.4  [GREEN]   Add 'meta' to Op type; OptionsPanel meta fields; App.svelte run() branch
- [ ] 22.5  [RED]     cmd_watermark Rust test
- [ ] 22.6  [GREEN]   Implement cmd_watermark; add watermark op to UI
- [ ] 22.7  [RED]     cmd_list_bookmarks + cmd_add_bookmark Rust tests
- [ ] 22.8  [GREEN]   Implement bookmark commands; add bookmarks op to UI with list/add sub-mode
- [ ] 22.9  [RED]     cmd_extract_images Rust test (output dir contains .jpg/.png files)
- [ ] 22.10 [GREEN]   Implement cmd_extract_images; add images op to UI; use pickOutputDir() from Phase 21
- [ ] 22.11 [REFACTOR] Deduplicate OptionsPanel — consider splitting into per-op sub-components if it exceeds 300 lines
- [ ] 22.12 [REVIEW]  All tests pass; all 4 new ops smoke-tested end-to-end; clippy clean
```

### Definition of Done

- [ ] Metadata, Watermark, Bookmarks, Image Extraction appear as sidebar ops
- [ ] Each op has appropriate form fields in OptionsPanel
- [ ] Metadata: current values loaded on file drop; updated on Run
- [ ] Bookmarks: list view shows existing bookmarks; add form appends one
- [ ] Image extraction: output dir is user-selected (folder picker)
- [ ] All Rust and frontend tests pass
- [ ] No ezpdf-core changes needed (all functions already exist)

---

## Phase 23: Distribution & Packaging

**Target:** v2.3

### Features

1. **Linux packaging** — `.deb` and `.AppImage` bundles built in CI

> **Skipped:** macOS code signing + notarization (requires Apple Developer Program), auto-updater (manual download from GitHub Releases is sufficient for personal use).

### Technical Approach

#### 23.1 — Linux Packaging

**Tauri v2** builds `.deb` and `.AppImage` natively with `cargo tauri build` on Linux. Add a Linux CI runner:

```yaml
# .github/workflows/release.yml
- name: Build Linux
  runs-on: ubuntu-22.04
  steps:
    - uses: actions/checkout@v4
    - name: Install system deps
      run: sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
    - uses: dtolnay/rust-toolchain@stable
    - uses: pnpm/action-setup@v4
    - run: cargo tauri build
    - uses: actions/upload-artifact@v4
      with:
        name: linux-packages
        path: |
          target/release/bundle/deb/*.deb
          target/release/bundle/appimage/*.AppImage
```

**`tauri.conf.json` Linux section:**
```json
{
  "bundle": {
    "linux": {
      "deb": { "depends": ["libwebkit2gtk-4.1-0", "libgtk-3-0"] }
    }
  }
}
```

### Tasks

```
- [ ] 23.1  [GREEN]   Add ubuntu-22.04 job to release.yml; install webkit2gtk system deps; build .deb + .AppImage
- [ ] 23.2  [REVIEW]  CI run produces .deb + .AppImage artifacts; .AppImage runs on Linux machine without install
```

### Definition of Done

- [ ] Linux: `.deb` installs cleanly on Ubuntu 22.04; `.AppImage` runs on Ubuntu 22.04
- [ ] All artifacts uploaded to GitHub Release automatically by CI
- [ ] No regressions in existing Rust or frontend tests

---

## Dependencies Between Phases

```
Phase 21 (UX Polish)
  └─ Phase 22 depends on Phase 21's output path picker (pickOutputDir for images)
       └─ Phase 23 depends on Phase 22 being complete (distributes a feature-complete app)
```

Phases should be implemented in order: 21 → 22 → 23.

---

## Sources & References

### Internal References

- Phase 20 complete implementation: `ezpdf-app/frontend/src/App.svelte`
- Existing command pattern: `ezpdf-app/src/commands.rs`
- Tauri invoke handler: `ezpdf-app/src/lib.rs:7-16`
- Dialog helpers: `ezpdf-app/frontend/src/lib/dialog.ts`
- Existing test patterns: `ezpdf-app/frontend/src/App.test.ts`
- Prior phases plan: `docs/plans/2026-03-13-001-feat-ezpdf-backlog-phases-plan.md`
- Desktop app brainstorm: `docs/brainstorms/2026-03-14-desktop-app-brainstorm.md`
- Tauri capabilities: `ezpdf-app/capabilities/default.json`
- Core ops already implemented: `ezpdf-core/src/metadata.rs`, `ezpdf-core/src/watermark.rs`, `ezpdf-core/src/bookmarks.rs`, `ezpdf-core/src/images.rs`

### External References

- Tauri v2 updater plugin: https://tauri.app/plugin/updater/
- Tauri v2 code signing: https://tauri.app/distribute/sign/macos/
- Tauri v2 Linux packaging: https://tauri.app/distribute/linux/
- GitHub Actions Tauri action: https://github.com/tauri-apps/tauri-action
