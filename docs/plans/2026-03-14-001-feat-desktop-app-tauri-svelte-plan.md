---
title: "feat: Desktop App — Tauri v2 + Svelte 5 (Phase 20)"
type: feat
status: active
date: 2026-03-14
origin: docs/brainstorms/2026-03-14-desktop-app-brainstorm.md
---

# feat: Desktop App — Tauri v2 + Svelte 5 (Phase 20)

## Overview

Build a native desktop application for macOS and Linux that wraps ezpdf's five core
operations (merge, split, remove, rotate, reorder) in a polished drag-and-drop GUI.
The app is implemented inside the existing `ezpdf-app` Cargo workspace member using
Tauri v2 for the Rust/native layer and Svelte 5 + Vite for the frontend.

**Stack:** Tauri v2 · Svelte 5 · Vite · Vitest · @testing-library/svelte
**Platforms:** macOS arm64, macOS x86_64, Linux x86_64
**Scope:** Core 5 operations only — merge, split, remove, rotate, reorder

## Problem Statement

The ezpdf CLI tool requires terminal proficiency. A drag-and-drop desktop app lowers
the barrier of entry, makes the tool accessible to non-technical users, and demonstrates
the reusability of `ezpdf-core` as a library. Users can drop PDFs onto the window,
pick an operation, set options, and hit Run — without touching a shell.

## Proposed Solution

Tauri v2 wraps `ezpdf-core` via `#[tauri::command]` IPC functions. A Svelte 5 frontend
renders the UI. The Rust backend is unit-tested with `cargo test -p ezpdf-app`; the
frontend components are tested with Vitest + `@testing-library/svelte`. The final
artefact is verified by `cargo tauri build` exiting 0.

(see brainstorm: docs/brainstorms/2026-03-14-desktop-app-brainstorm.md)

## Technical Approach

### Architecture

```
User drops PDF(s)
        │
        ▼
  Svelte 5 frontend (WebView)
  ┌──────────────────────────────┐
  │  App.svelte                  │
  │  ├─ Sidebar (op selection)   │
  │  ├─ DropZone (files)         │
  │  ├─ FileList (list + reorder)│
  │  └─ OptionsPanel (per-op)    │
  │                              │
  │  invoke('cmd_merge', {...})   │
  └──────────┬───────────────────┘
             │ Tauri IPC (JSON)
             ▼
  Tauri Rust commands (ezpdf-app/src/lib.rs)
  ┌──────────────────────────────┐
  │  cmd_merge / cmd_split /     │
  │  cmd_remove / cmd_rotate /   │
  │  cmd_reorder                 │
  └──────────┬───────────────────┘
             │ direct fn calls
             ▼
  ezpdf-core (ezpdf-core/src/lib.rs)
  merge() / split_range() / split_each()
  remove() / rotate() / reorder()
```

### Workspace Layout After Phase 20

```
ezpdf-app/
├── Cargo.toml              ← add tauri, tauri-plugin-fs, tauri-plugin-dialog, ezpdf-core deps
├── build.rs                ← tauri_build::build()
├── tauri.conf.json         ← productName, bundle, devUrl, frontendDist
├── capabilities/
│   └── default.json        ← fs:allow-read-file, dialog:allow-open, dialog:allow-save
├── src/
│   ├── main.rs             ← tauri::Builder::default()...run()
│   └── lib.rs              ← #[tauri::command] functions + #[cfg(test)]
└── frontend/
    ├── package.json        ← @tauri-apps/api ^2, svelte ^5, vite ^6
    ├── vite.config.ts      ← port 1420, clearScreen false, Tauri env vars
    ├── vitest.config.ts    ← jsdom, svelteTesting(), @testing-library/jest-dom
    ├── src/
    │   ├── main.ts
    │   ├── App.svelte      ← root: Sidebar + DropZone + OptionsPanel + status bar
    │   ├── lib/
    │   │   └── tauri.ts    ← typed invoke() wrappers with Result<string, string>
    │   └── components/
    │       ├── Sidebar.svelte      ← op selection, highlights active
    │       ├── DropZone.svelte     ← drag-and-drop, calls onDragDropEvent
    │       ├── FileList.svelte     ← ordered list, remove button per file
    │       └── OptionsPanel.svelte ← op-specific form fields
    └── src/
        └── test-setup.ts   ← import '@testing-library/jest-dom/vitest'
```

### Tauri Command Signatures

```rust
// ezpdf-app/src/lib.rs

#[tauri::command]
pub fn cmd_merge(inputs: Vec<String>, output: String) -> Result<String, String>

#[tauri::command]
pub fn cmd_split_range(input: String, range: String, output: String) -> Result<String, String>

#[tauri::command]
pub fn cmd_split_each(input: String, output_dir: String) -> Result<String, String>

#[tauri::command]
pub fn cmd_remove(input: String, pages: String, output: String) -> Result<String, String>

#[tauri::command]
pub fn cmd_rotate(input: String, degrees: i32, pages: Option<String>, output: String) -> Result<String, String>

#[tauri::command]
pub fn cmd_reorder(input: String, order: String, output: String) -> Result<String, String>
```

All commands:
- Accept `String` paths (Tauri IPC serialises via JSON)
- Return `Result<String, String>` — Ok(success_message) or Err(error_message)
- Call `ezpdf_core::*` functions directly (same ones the CLI uses)
- Are testable without a Tauri runtime by calling them as plain Rust functions

### Tauri Config Files

**`ezpdf-app/tauri.conf.json`:**
```json
{
  "productName": "ezpdf",
  "version": "0.1.0",
  "identifier": "com.ezpdf.app",
  "build": {
    "beforeDevCommand": "pnpm --prefix frontend dev",
    "beforeBuildCommand": "pnpm --prefix frontend build",
    "devUrl": "http://localhost:1420",
    "frontendDist": "frontend/dist"
  },
  "bundle": { "active": true, "targets": "all", "icon": [] },
  "app": {
    "windows": [{ "title": "ezpdf", "width": 900, "height": 600, "dragDropEnabled": true }],
    "security": { "csp": null }
  }
}
```

**`ezpdf-app/capabilities/default.json`:**
```json
{
  "$schema": "gen/schemas/desktop-schema.json",
  "identifier": "default",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "dialog:allow-open",
    "dialog:allow-save",
    "fs:allow-read-file",
    "fs:allow-write-file"
  ]
}
```

### Frontend Testing Strategy

Tauri's `invoke()` and `getCurrentWebview()` are unavailable in Vitest's jsdom
environment. Components must abstract these behind wrapper functions that can be
mocked.

**`frontend/src/lib/tauri.ts`** exports typed wrappers:
```typescript
import { invoke } from '@tauri-apps/api/core';

export async function cmdMerge(inputs: string[], output: string): Promise<string> {
  return invoke<string>('cmd_merge', { inputs, output });
}
// ...etc for each command
```

**`frontend/src/lib/dnd.ts`** exports file-drop registration:
```typescript
import { getCurrentWebview } from '@tauri-apps/api/webview';
export async function onFileDrop(handler: (paths: string[]) => void) { ... }
```

In Vitest tests, mock both modules:
```typescript
vi.mock('$lib/tauri', () => ({ cmdMerge: vi.fn().mockResolvedValue('Merged → out.pdf') }))
vi.mock('$lib/dnd', () => ({ onFileDrop: vi.fn() }))
```

This allows every Svelte component to be tested in pure jsdom without a Tauri instance.

### Drag-and-Drop Implementation

Tauri v2 uses `getCurrentWebview().onDragDropEvent()` (not the native web drop event).
The event payload has `.type` = `'enter' | 'over' | 'drop' | 'leave'` and `.paths: string[]`.

**Known quirk:** `onDragDropEvent` fires twice per drop. Deduplicate by tracking the
last-processed paths array.

```typescript
// frontend/src/lib/dnd.ts
import { getCurrentWebview } from '@tauri-apps/api/webview';
let lastPaths: string[] = [];

export async function onFileDrop(handler: (paths: string[]) => void) {
  return getCurrentWebview().onDragDropEvent((event) => {
    if (event.payload.type === 'drop') {
      const paths = event.payload.paths;
      if (JSON.stringify(paths) !== JSON.stringify(lastPaths)) {
        lastPaths = paths;
        handler(paths.filter(p => p.endsWith('.pdf')));
      }
    }
  });
}
```

### Output Path Logic

```typescript
// Auto-name alongside first input file
function defaultOutput(inputs: string[], op: Operation): string {
  const dir = inputs[0].replace(/[^/\\]+$/, '');  // dirname
  const base = inputs[0].replace(/^.*[/\\]/, '').replace(/\.pdf$/i, '');
  return `${dir}${base}-${op}.pdf`;
}
// e.g. /home/user/report.pdf + merge → /home/user/report-merged.pdf
// For split_each: returns dir (user picks folder via dialog)
```

## UI Layout (ASCII Mockup)

```
┌──────────────────────────────────────────────────────┐
│  ezpdf                                    [─][□][×]  │
├─────────────┬────────────────────────────────────────┤
│             │                                        │
│  ● Merge    │     Drop PDF files here               │
│  ○ Split    │     or click to browse                │
│  ○ Remove   │                                        │
│  ○ Rotate   │   [report.pdf ×]  [appendix.pdf ×]   │
│  ○ Reorder  │                                        │
│             ├────────────────────────────────────────┤
│             │  Output: /same/folder/report-merged.pdf│
│             │                              [Save As…]│
│             │                                        │
│             │  [ ▶  Run Merge  ]                    │
│             │                                        │
│             │  ✓ Merged 2 files → report-merged.pdf │
│             │    (1.8 MB)                            │
└─────────────┴────────────────────────────────────────┘
```

**Split operation** shows a radio toggle:
```
  Mode: (●) Extract range  ( ) Burst all pages
  Range: [1-5          ]
```

**Rotate operation** shows a degree selector:
```
  Degrees: [90 ▾]
  Pages:   [all      ]  (optional, e.g. 1,3,5)
```

## System-Wide Impact

### Interaction Graph

Drop event → `onFileDrop(paths)` → state update → re-render FileList →
user clicks Run → `cmdMerge(inputs, output)` → `invoke('cmd_merge')` →
Tauri IPC → `cmd_merge(inputs, output)` → `ezpdf_core::merge()` →
`lopdf::Document::save_to()` → file on disk → Ok(message) →
IPC response → status bar update.

### Error Propagation

`ezpdf_core::merge()` returns `Err(EzPdfError)` → `cmd_merge` converts via
`.map_err(|e| e.to_string())` → Tauri serialises as `{"error": "..."}` →
`invoke()` rejects Promise → `.catch()` handler in App.svelte → status bar
shows error string in red.

No errors are swallowed silently. All Tauri command functions return
`Result<String, String>` — there are no `unwrap()` calls.

### State Lifecycle Risks

- File list is ephemeral state in the Svelte component — no persistence risk.
- Output file write: if `ezpdf_core` fails mid-write, the output file may be
  partial. Risk is inherent in the underlying library (same as CLI behaviour).
- Temporary files: none created by the GUI layer itself.

### API Surface Parity

The desktop app exposes a subset (5 of 11) of the CLI commands. This is
intentional per the brainstorm scope decision. Future phases may add the
remaining 6 operations.

### Integration Test Scenarios

1. User drops a non-PDF file → only `.pdf` files are accepted, others silently
   filtered from the list.
2. User clicks Run with no files loaded → button is disabled / shows validation
   error in the UI; no Tauri command is invoked.
3. `cmd_merge` returns `Err("encrypted PDF")` → status bar shows the error;
   file list remains unchanged so user can try again with a different file.
4. Split with burst mode: output must be a folder → dialog shows folder picker
   (not file picker); `cmd_split_each` receives the folder path.
5. Output file already exists → app writes over it (same behaviour as CLI);
   this is acceptable since users chose the path explicitly.

## Implementation Phases (Ralph Tasks)

The phase is split into two toolchains: Rust backend first, Svelte frontend second.
Ralph verifies each sub-phase with the appropriate test runner before advancing.

---

### Definition of Done (Phase 20)

- [ ] `cargo test -p ezpdf-app` passes — all Tauri command unit tests green
- [ ] `cd ezpdf-app/frontend && pnpm test` passes — all Vitest component tests green
- [ ] `cd ezpdf-app && cargo tauri build` exits 0 — app bundle is shippable
- [ ] All 5 operations work end-to-end in the running app (manual smoke test)
- [ ] App handles drag-and-drop of PDF files on macOS and Linux
- [ ] Error messages appear in UI when operations fail (e.g. encrypted PDF)
- [ ] No `unwrap()` / `expect()` in `ezpdf-app/src/lib.rs`
- [ ] `cargo clippy --workspace -- -D warnings` passes
- [ ] `cargo fmt --check` passes

---

### Phase 20 Tasks

```
## Phase 20: Desktop App (Tauri v2 + Svelte 5)

### Definition of Done
- [ ] `cargo test -p ezpdf-app` passes — all 6 Tauri command tests green
- [ ] `cd ezpdf-app/frontend && pnpm test` passes — all Vitest component tests green
- [ ] `cd ezpdf-app && cargo tauri build` exits 0
- [ ] Manual smoke test: all 5 ops work in the running app
- [ ] No unwrap/expect in lib.rs; clippy/fmt clean

### Tasks

- [ ] **20.1 [SETUP]** Scaffold Tauri v2 Rust layer in ezpdf-app/
      - Update ezpdf-app/Cargo.toml: add tauri="2", tauri-plugin-fs="2",
        tauri-plugin-dialog="2", ezpdf-core path dep; add [build-dependencies]
        tauri-build="2"; add [dev-dependencies] tauri with "test" feature
      - Create ezpdf-app/build.rs: `fn main() { tauri_build::build() }`
      - Create ezpdf-app/tauri.conf.json with productName="ezpdf",
        identifier="com.ezpdf.app", devUrl="http://localhost:1420",
        frontendDist="frontend/dist", window 900×600, dragDropEnabled=true
      - Create ezpdf-app/capabilities/default.json with core:default,
        dialog:allow-open, dialog:allow-save, fs:allow-read-file, fs:allow-write-file
      - Replace ezpdf-app/src/lib.rs stub with empty pub fn run() scaffold
      - Replace ezpdf-app/src/main.rs (or create) with tauri::Builder invocation
      - Verify: `cargo build -p ezpdf-app` exits 0

- [ ] **20.2 [RED]** Write 6 failing Rust unit tests for Tauri commands
      - Test file: ezpdf-app/src/lib.rs #[cfg(test)] module
      - Tests (call command functions directly — no Tauri runtime needed):
        1. cmd_merge_combines_files: merge 3page.pdf+5page.pdf → 8 pages
        2. cmd_merge_missing_input_returns_err
        3. cmd_split_range_produces_correct_pages
        4. cmd_remove_removes_pages
        5. cmd_rotate_rotates_all_pages
        6. cmd_reorder_changes_page_order
      - Run `cargo test -p ezpdf-app` — all 6 FAIL (commands not yet implemented)
      - Commit: "test(app): failing tests for 6 Tauri commands"

- [ ] **20.3 [GREEN]** Implement 6 Tauri commands — minimum to pass the tests
      - Add to ezpdf-app/src/lib.rs:
        cmd_merge, cmd_split_range, cmd_split_each, cmd_remove,
        cmd_rotate, cmd_reorder — each converts String paths to &Path,
        calls ezpdf_core fn, returns Ok(success_msg) or Err(e.to_string())
      - Register all commands in invoke_handler in run()
      - Register tauri-plugin-fs and tauri-plugin-dialog in Builder
      - Run `cargo test -p ezpdf-app` — all 6 PASS
      - Commit: "feat(app): Tauri commands wrapping ezpdf-core (6 tests pass)"

- [ ] **20.4 [SETUP]** Scaffold Svelte 5 + Vite + Vitest frontend
      - Create ezpdf-app/frontend/ with:
        - package.json: scripts dev/build/test/tauri; deps: @tauri-apps/api ^2,
          @tauri-apps/plugin-fs ^2, @tauri-apps/plugin-dialog ^2, svelte ^5;
          devDeps: @sveltejs/vite-plugin-svelte ^4, @tauri-apps/cli ^2,
          typescript ^5, vite ^6, vitest, @testing-library/svelte,
          @testing-library/jest-dom, jsdom
        - vite.config.ts: port 1420, clearScreen: false, Tauri env prefixes,
          svelte() plugin
        - vitest.config.ts: environment: 'jsdom', setupFiles: ['src/test-setup.ts'],
          svelteTesting() plugin
        - src/test-setup.ts: import '@testing-library/jest-dom/vitest'
        - src/main.ts: mount App
        - src/App.svelte: placeholder <h1>ezpdf</h1>
        - src/lib/tauri.ts: typed invoke() wrappers (cmdMerge etc)
        - src/lib/dnd.ts: onFileDrop() wrapper around getCurrentWebview().onDragDropEvent()
          with deduplication (lastPaths guard)
        - src/components/Sidebar.svelte: placeholder
        - src/components/DropZone.svelte: placeholder
        - src/components/FileList.svelte: placeholder
        - src/components/OptionsPanel.svelte: placeholder
      - Run `cd ezpdf-app/frontend && pnpm install` — succeeds
      - Run `pnpm build` — empty app builds
      - Commit: "chore(app): Svelte 5 + Vite + Vitest scaffold"

- [ ] **20.5 [RED]** Vitest tests for DropZone component
      - File: ezpdf-app/frontend/src/components/DropZone.test.ts
      - Mock: vi.mock('../lib/dnd', ...) to simulate file drop
      - Tests:
        1. renders drop zone with instructions text
        2. emits 'filesAdded' when onFileDrop callback fires with .pdf paths
        3. filters out non-PDF paths from drop event
        4. shows dropped filenames in the component
      - Run `pnpm test` — all 4 FAIL
      - Commit: "test(app): failing DropZone component tests"

- [ ] **20.6 [GREEN]** Implement DropZone component
      - ezpdf-app/frontend/src/components/DropZone.svelte:
        - On mount, call onFileDrop(), filter to .pdf only, dispatch 'filesAdded'
        - Show visual drop target with text "Drop PDF files here"
        - Show list of added file basenames with × buttons
        - On destroy, call unlisten()
      - Run `pnpm test` — DropZone tests PASS
      - Commit: "feat(app): DropZone component with Tauri drag-and-drop"

- [ ] **20.7 [RED]** Vitest tests for Sidebar + OptionsPanel
      - Sidebar tests (ezpdf-app/frontend/src/components/Sidebar.test.ts):
        1. renders 5 operation buttons: Merge, Split, Remove, Rotate, Reorder
        2. clicking an op dispatches 'opSelected' event with op name
        3. selected op has 'active' CSS class
      - OptionsPanel tests (ezpdf-app/frontend/src/components/OptionsPanel.test.ts):
        1. merge panel: no extra options (just file list)
        2. split panel: shows mode toggle (range/burst) and range input
        3. remove panel: shows pages input
        4. rotate panel: shows degrees selector (90/180/270/-90) and pages input
        5. reorder panel: shows order input
      - Run `pnpm test` — all FAIL
      - Commit: "test(app): failing Sidebar and OptionsPanel tests"

- [ ] **20.8 [GREEN]** Implement Sidebar + OptionsPanel components
      - Sidebar.svelte: 5 op buttons, dispatch 'opSelected', active class on selected
      - OptionsPanel.svelte: conditional rendering by `op` prop:
        - merge: nothing extra
        - split: radio (range/burst) + range input (shown only for range mode)
        - remove: pages text input (e.g. "2,4-6")
        - rotate: <select> for degrees + optional pages input
        - reorder: order text input (e.g. "3,1,2")
      - Run `pnpm test` — Sidebar + OptionsPanel tests PASS
      - Commit: "feat(app): Sidebar and OptionsPanel components"

- [ ] **20.9 [RED]** Vitest tests for App integration (invoke mocking)
      - File: ezpdf-app/frontend/src/App.test.ts
      - Mock: vi.mock('../lib/tauri', ...) with cmdMerge, cmdSplitRange,
        cmdSplitEach, cmdRemove, cmdRotate, cmdReorder all as vi.fn()
      - Tests:
        1. Run button is disabled when no files are loaded
        2. Run button is enabled after files are added
        3. Clicking Run with merge op calls cmdMerge with correct args
        4. Success result appears in status bar
        5. Error result appears in status bar with error text
        6. Output path defaults to same folder as first input
      - Run `pnpm test` — all 6 FAIL
      - Commit: "test(app): failing App integration tests"

- [ ] **20.10 [GREEN]** Implement App.svelte wiring
      - App.svelte:
        - State: files[], selectedOp, options{}, status{type, message}
        - Compose: <Sidebar> + <DropZone> + <FileList> + <OptionsPanel>
        - On filesAdded: merge into files state
        - Compute output path from first input + op name (defaultOutput())
        - Run button: disabled when files.length === 0 (or 0 for merge)
        - On Run: call appropriate cmdX from lib/tauri.ts; set status on
          resolve/reject; show message + file size if success
        - For split_each burst mode: use @tauri-apps/plugin-dialog to pick output folder
      - Run `pnpm test` — all App tests PASS
      - Run `cd .. && cargo test -p ezpdf-app` — still passing
      - Commit: "feat(app): App.svelte wires operations to Tauri commands"

- [ ] **20.11 [REFACTOR]** Polish, error handling, and cleanup
      - Add CSS: sidebar highlight, drop zone hover state, status bar colour
        (green success / red error), button loading state during Run
      - Validate: if no files → show "Add at least one file" instead of disabling
        silently; if required fields empty → inline validation message
      - Remove any console.log statements
      - Ensure all Rust lib.rs functions have no unwrap/expect
      - Run fmt: `cargo fmt --check` + prettier/eslint if configured
      - Run `cargo clippy --workspace -- -D warnings` — clean
      - Run `pnpm test` — still passing
      - Commit: "refactor(app): UI polish, validation, no console.log"

- [ ] **20.12 [REVIEW]** Validate Definition of Done
      - Run `cargo test -p ezpdf-app` → paste output showing 6 tests pass
      - Run `cd ezpdf-app/frontend && pnpm test` → paste output showing all tests pass
      - Run `cd ezpdf-app && cargo tauri build` → must exit 0
      - Manual smoke test: launch app, drop a PDF, run merge → verify output file created
      - Append Phase 20 section to progress.md
      - Commit: "chore: Phase 20 complete — desktop app (Tauri v2 + Svelte 5)"
      - Output completion signal: <promise>EZPDF BACKLOG COMPLETE</promise>
```

## Acceptance Criteria

### Functional Requirements

- [ ] User can drag-and-drop PDF files onto the app window (macOS + Linux)
- [ ] Merge: 2+ PDFs combined into one output file
- [ ] Split range: extracts page range from single PDF
- [ ] Split burst: splits into one-file-per-page, saves to chosen folder
- [ ] Remove: deletes specified pages from a PDF
- [ ] Rotate: rotates all or specified pages by chosen degrees
- [ ] Reorder: rearranges pages in a specified order
- [ ] Output file defaults to same folder as first input + `-<op>.pdf` suffix
- [ ] "Save As…" button lets user pick a different output location
- [ ] Error messages shown in UI when operations fail (no silent failures)
- [ ] Run button disabled when no files are loaded

### Non-Functional Requirements

- [ ] App bundle produced by `cargo tauri build` (macOS .app, Linux AppImage)
- [ ] No `unwrap()` / `expect()` in `ezpdf-app/src/lib.rs`
- [ ] All Tauri commands return `Result<String, String>` (no panics)
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] `cargo fmt --check` clean

### Quality Gates

- [ ] `cargo test -p ezpdf-app` — 6+ Rust unit tests pass
- [ ] `pnpm test` (inside `frontend/`) — Vitest component tests pass
- [ ] `cargo tauri build` exits 0

## Dependencies & Risks

| Dependency | Risk | Mitigation |
|---|---|---|
| Tauri v2 (new major version) | API differences from v1 examples online | Plan uses v2-specific API throughout; research confirmed v2 patterns |
| `@testing-library/svelte` Svelte 5 support | Marked "experimental" for Svelte 5 | Use `svelteTesting()` vite plugin as per official docs; fallback: mock components directly |
| jsdom + Tauri mocks | `invoke()` not available in jsdom | `lib/tauri.ts` abstraction layer makes mocking clean |
| `getCurrentWebview().onDragDropEvent` fires twice | Double-processing of drops | `lastPaths` deduplication guard in `dnd.ts` |
| Cargo workspace + Tauri CLI | `cargo tauri build` must be run from `ezpdf-app/` | Task 20.12 specifies `cd ezpdf-app &&` prefix |
| macOS notarization | Required for distribution | Out of scope for Phase 20; distribution workflow is a separate future phase |

## Alternative Approaches (Why Rejected)

See brainstorm: `docs/brainstorms/2026-03-14-desktop-app-brainstorm.md` — Alternatives section.

Summary:
- **egui / iced (pure Rust)**: Rejected — non-native appearance, limited layout control
- **Electron**: Rejected — 200 MB+ bundle size, defeats "fast + lean" brand
- **Tauri + vanilla HTML**: Rejected — no component model, harder to maintain

## Future Considerations

- Phase 21+: Add remaining 6 operations (info, meta, watermark, bookmarks, images, optimize)
- Phase 22+: Distribution workflow (macOS notarization, Linux AppImage/deb packaging)
- Phase 23+: Windows support

## Documentation Plan

After Phase 20 completes:
- Update `docs/user-manual.md` to add a "Desktop App" section
- Update `README.md` to mention the desktop app and how to build it

## Sources & References

### Origin

- **Brainstorm:** [docs/brainstorms/2026-03-14-desktop-app-brainstorm.md](../brainstorms/2026-03-14-desktop-app-brainstorm.md)
  Key decisions carried forward: (1) Tauri v2 + Svelte 5 over egui/Electron; (2) Core 5 ops only; (3) Drag-and-drop sidebar UI; (4) `lib/tauri.ts` abstraction for testability

### Internal References

- ezpdf-core public API: `ezpdf-core/src/lib.rs`
- CLI command patterns: `ezpdf-cli/src/commands/*.rs`
- Task plan conventions: `task_plan.md` (existing phase structure)
- Ralph loop rules: `PROMPT.md`

### External References

- Tauri v2 IPC commands: https://v2.tauri.app/develop/calling-rust/
- Tauri v2 capabilities: https://v2.tauri.app/security/capabilities/
- Tauri v2 drag-drop API: https://v2.tauri.app/reference/javascript/api/namespacewebviewwindow/
- Tauri v2 testing: https://v2.tauri.app/develop/tests/
- `@testing-library/svelte` + Svelte 5: https://testing-library.com/docs/svelte-testing-library/setup/
- `vite.config.ts` for Tauri: https://v2.tauri.app/start/frontend/vite/
- `tauri-plugin-fs` + `tauri-plugin-dialog`: https://v2.tauri.app/plugin/file-system/
