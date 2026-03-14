---
title: feat: Plan EZPDF Backlog as Trackable Phases (v1.1 → v2)
type: feat
status: active
date: 2026-03-13
---

# EZPDF Backlog Phases Plan

## Overview

Add backlog items B1–B10 as structured phases (11–20) in `task_plan.md` and `progress.md` so the ralph autonomous loop can execute them without manual intervention. No changes to `ralph-loop.sh` or `PROMPT.md` are required — the existing completion signal fires when all tasks are checked, including new ones.

## Background

EZPDF v1 (v0.1.1) shipped on 2026-03-13 with 5 core PDF operations, CLI polish, benchmarks, and a GitHub release with 4 binary artifacts. The backlog contains 10 items ranging from simple commands to a full desktop app. The ralph loop and `task_plan.md` format are battle-tested — the same system needs to drive v1.1, v1.2, and v2 work.

## Version Groupings

| Version | Phases | Backlog IDs | Items |
|---------|--------|-------------|-------|
| v1.1 | 11–12 | B4, B1 | `ezpdf info` command, batch operations |
| v1.2 | 13–14 | B5, B6 | metadata read/write, Windows support |
| v2 | 15–20 | B2, B9, B10, B7, B8, B3 | encrypted PDFs, watermark, bookmarks, images, linearize, desktop app |

## Files to Change

| File | Change |
|------|--------|
| `task_plan.md` | Append Phases 11–20 with full [RED]/[GREEN]/[REFACTOR]/[REVIEW] tasks |
| `task_plan.md` | Remove old backlog table (items now live as phases) |
| `ralph-loop.sh` | Update `COMPLETION_SIGNAL` to `EZPDF BACKLOG COMPLETE` |
| `PROMPT.md` | Update completion signal text to match |
| `progress.md` | Append a planning note marking backlog phases as added |

---

## Phase 11: `ezpdf info` Command

**Target:** v1.1
**Backlog:** B4 — "ezpdf info: show page count, dimensions, metadata"

### Definition of Done

- [ ] `cargo test -p ezpdf-core info` passes
- [ ] `cargo test -p ezpdf-cli info` passes
- [ ] `ezpdf info input.pdf` prints page count, per-page dimensions, and document metadata fields
- [ ] `ezpdf info input.pdf --json` outputs valid JSON

### Key Technical Notes

- **`PdfInfo` struct fields:** `page_count: u32`, `dimensions: Vec<(f64, f64)>` (width × height in points, one per page), `title: Option<String>`, `author: Option<String>`, `subject: Option<String>`, `keywords: Option<String>`, `creator: Option<String>`, `producer: Option<String>`
- **Page dimensions:** For each page id from `doc.get_pages()`, read the `/MediaBox` array `[x1, y1, x2, y2]` → `width = x2 - x1`, `height = y2 - y1`. Fall back to `/CropBox` then `/ArtBox` if `/MediaBox` is absent on the page dict (it may be inherited from `/Pages`).
- **Metadata:** `doc.trailer.get(b"Info")` → follow the object reference → get the `/Info` dictionary → read each field as a `PDFString` or `Name`, decode from PDFDocEncoding or UTF-16BE.
- **lopdf gotcha:** Dictionary values for text fields are `Object::String(bytes, StringFormat::*)`. Use `std::str::from_utf8` for ASCII/PDFDocEncoding; for UTF-16BE strings (starting with BOM `\xFE\xFF`), decode accordingly.

### Tasks

```
- [ ] **11.1 [RED]** Write failing tests for `ezpdf_core::info(input: &Path) -> Result<PdfInfo, EzPdfError>`.
  Tests: (a) info on 3page.pdf → page_count == 3; (b) dimensions vec has 3 entries, each > 0;
  (c) info on nonexistent file → Io error; (d) info on encrypted.pdf fixture → EncryptedPdf error.
  Also define `PdfInfo` struct in tests with the fields listed above (it won't exist yet).
  Run `cargo test -p ezpdf-core` — must FAIL.

- [ ] **11.2 [GREEN]** Create `ezpdf-core/src/info.rs`.
  Define `PdfInfo` struct (derive Debug, PartialEq). Implement `pub fn info(input: &Path) -> Result<PdfInfo, EzPdfError>`.
  Use `load_doc(input)?` for loading and encrypted detection.
  Extract page count via `doc.get_pages().len() as u32`.
  Extract dimensions: iterate `doc.get_pages()` (sorted by page number), for each page object id,
  call `doc.get_object(id)?.as_dict()?`, then look for `/MediaBox` in page dict (may need to walk
  up to parent /Pages dict if not present on page). Parse as `[x0, y0, x1, y1]` floats.
  Extract metadata: follow `doc.trailer.get(b"Info")` reference chain to the Info dictionary,
  then read each key as an Option<String>.
  Export from `lib.rs`. All tests must PASS.

- [ ] **11.3 [RED]** Write failing CLI tests for `ezpdf info`:
  (a) `ezpdf info 3page.pdf` exits 0, stdout contains "Pages: 3";
  (b) `ezpdf info 3page.pdf --json` exits 0, stdout is parseable JSON with `page_count` field;
  (c) `ezpdf info nonexistent.pdf` exits 1, stderr contains "Error:".
  Run tests — must FAIL.

- [ ] **11.4 [GREEN]** Create `ezpdf-cli/src/commands/info.rs` with `InfoArgs { file: PathBuf, json: bool }`.
  Wire as `ezpdf info` subcommand. For normal output: print "File: {}", "Pages: {}", dimensions
  table (Page | Width pt | Height pt), and metadata fields (skip None values).
  For --json: serialize PdfInfo to JSON using `serde_json` (add `serde` feature to `ezpdf-core`
  and `serde_json` to `ezpdf-cli` dev + runtime deps). All tests must PASS.

- [ ] **11.5 [REFACTOR]** Format dimensions output: detect common paper sizes (A4 = 595×842,
  Letter = 612×792) and append the size name when matched (within 2pt tolerance).
  Add `--pages` flag to info to show dimensions for specific pages only (uses page_range::parse).
  Clippy clean.

- [ ] **11.6 [REVIEW]** Run `cargo test --workspace`. Manual demo: `ezpdf info` on a real PDF.
  Check DoD. Commit with message `feat: ezpdf info command (Phase 11)`. Update `progress.md`.
```

---

## Phase 12: Batch Operations

**Target:** v1.1
**Backlog:** B1 — "Batch operations (`--batch` flag for directory processing)"

### Definition of Done

- [ ] `--batch` flag works on: `rotate`, `remove`, `reorder` (apply operation to each PDF in directory)
- [ ] `merge --batch` merges all PDFs in a directory into one output file
- [ ] `split --batch` splits each PDF in a directory into its own output subdirectory
- [ ] Progress bar shown for batch runs with > 1 file

### Key Technical Notes

- **Semantics per command:**
  - `rotate --batch DIR/ 90 -o OUT_DIR/` → rotate each PDF, save as `OUT_DIR/<original_name>.pdf`
  - `remove --batch DIR/ 1 -o OUT_DIR/` → remove page 1 from each PDF, save to OUT_DIR
  - `reorder --batch DIR/ "2,1,3" -o OUT_DIR/` → reorder each PDF
  - `merge --batch DIR/ -o OUT_FILE.pdf` → merge all PDFs in DIR into one file (sorted by filename)
  - `split --batch DIR/ 1-3 -o OUT_DIR/` → split each PDF, save chunks to `OUT_DIR/<stem>/part.pdf`
- **Core-level helper:** Add `pub fn collect_pdf_inputs(dir: &Path) -> Result<Vec<PathBuf>, EzPdfError>` in a new `ezpdf-core/src/batch.rs` — returns all `.pdf` files in a directory sorted alphabetically.
- **CLI pattern:** Each existing `Args` struct gets `batch: bool` added to clap. When `batch = true`, the positional input argument is treated as a directory rather than a single file.
- **Output dir creation:** Use `std::fs::create_dir_all` for the output directory.

### Tasks

```
- [ ] **12.1 [RED]** Write failing tests for `ezpdf_core::batch::collect_pdf_inputs(dir: &Path) -> Result<Vec<PathBuf>, EzPdfError>`.
  Tests: (a) dir with 3 PDF files → returns 3 sorted paths; (b) dir with mixed files → only .pdf included;
  (c) nonexistent dir → Io error; (d) empty dir → empty vec (not an error).
  Run `cargo test -p ezpdf-core` — must FAIL.

- [ ] **12.2 [GREEN]** Create `ezpdf-core/src/batch.rs`. Implement `collect_pdf_inputs` using
  `std::fs::read_dir`, filtering entries with `.extension() == Some("pdf")`, sorting by filename.
  Export from `lib.rs`. All tests must PASS.

- [ ] **12.3 [RED]** Write failing CLI tests for `--batch` flag on each command:
  (a) `ezpdf rotate --batch fixtures_dir/ 90 -o out_dir/` → exits 0, output dir contains N PDFs;
  (b) `ezpdf remove --batch fixtures_dir/ 1 -o out_dir/` → exits 0;
  (c) `ezpdf merge --batch fixtures_dir/ -o out.pdf` → exits 0, output is single PDF;
  (d) `--batch` with nonexistent dir → exits 1 with "Error:".
  Use tempdir for output. Run — must FAIL.

- [ ] **12.4 [GREEN]** Add `--batch` flag to all 5 command Args structs. Add batch dispatch logic
  to each command handler: if `batch` is true, call `collect_pdf_inputs` on the input path,
  then loop over each file applying the operation. For `merge --batch`, collect all files and
  call `merge()` once with all inputs. For `split --batch`, create a subdirectory per input file.
  Show progress using `indicatif` MultiProgress when batch has > 1 file. All tests must PASS.

- [ ] **12.5 [REFACTOR]** Extract a `run_batch<F>(inputs: Vec<PathBuf>, out_dir: &Path, op: F)`
  helper in the CLI to reduce duplication across the 4 independent-file commands. Add `--dry-run`
  flag that prints what would be done without executing. Clippy clean.

- [ ] **12.6 [REVIEW]** Run `cargo test --workspace`. Manual demo: batch rotate a directory of PDFs.
  Check DoD. Commit `feat: batch operations --batch flag (Phase 12)`. Update `progress.md`.
```

---

## Phase 13: PDF Metadata Read/Write

**Target:** v1.2
**Backlog:** B5 — "PDF metadata read/write (title, author, keywords)"

### Definition of Done

- [ ] `ezpdf meta get input.pdf` prints all metadata fields
- [ ] `ezpdf meta set input.pdf --title "..." -o output.pdf` updates selected fields and saves
- [ ] Round-trip: `set` then `get` returns the values that were set

### Key Technical Notes

- **New subcommand:** `ezpdf meta` with two sub-subcommands: `get` and `set`
- **Core API:**
  - `ezpdf_core::get_metadata(input: &Path) -> Result<PdfMetadata, EzPdfError>` (re-uses parts of Phase 11's info implementation)
  - `ezpdf_core::set_metadata(input: &Path, updates: MetadataUpdate, output: &Path) -> Result<(), EzPdfError>`
- **`MetadataUpdate` struct:** All fields `Option<String>` — only Some fields are updated; None fields are left unchanged. Add `clear_all: bool` flag to wipe all metadata.
- **lopdf writing:** Get the Info dict object id from trailer. If none exists, create a new `Dictionary` object and add its id to the trailer under `Info`. Set each field using `dict.set(b"Title", Object::string_literal(value))`.

### Tasks (abbreviated — expand to full [RED]→[GREEN]→[REFACTOR]→[REVIEW] when implementing)

```
- [ ] **13.1 [RED]** Failing tests for get_metadata / MetadataUpdate struct
- [ ] **13.2 [GREEN]** Create `ezpdf-core/src/metadata.rs`
- [ ] **13.3 [RED]** Failing CLI tests for `ezpdf meta get` and `ezpdf meta set`
- [ ] **13.4 [GREEN]** Create `ezpdf-cli/src/commands/meta.rs` with nested subcommands
- [ ] **13.5 [REFACTOR]** --json flag for `meta get`, validate field name enum for `meta set`
- [ ] **13.6 [REVIEW]** DoD check, round-trip test, commit, progress update
```

---

## Phase 14: Windows Support

**Target:** v1.2
**Backlog:** B6 — "Windows support"

### Definition of Done

- [ ] `cargo build --target x86_64-pc-windows-msvc` succeeds (or via cross)
- [ ] GitHub Actions CI runs on `windows-latest`
- [ ] Release workflow produces `ezpdf-v{version}-x86_64-pc-windows-msvc.zip`
- [ ] All existing 66 tests pass on Windows (path separator handling)

### Key Technical Notes

- **CI addition:** Add `windows-latest` to matrix in `.github/workflows/ci.yml`
- **Release addition:** Add `x86_64-pc-windows-msvc` target to `.github/workflows/release.yml`. Use `windows-latest` runner. Binary has `.exe` extension — strip step differs. Package as `.zip` instead of `.tar.gz`.
- **Code issues to check:** Any path construction using `/` literals (use `Path::join` instead), any `tempdir` paths with Unix assumptions, any `assert_cmd` tests relying on Unix exit signals.
- **Windows binary:** `strip` may not be available; use `llvm-strip` or skip stripping.
- **Scoop bucket** (optional): Add Scoop manifest for Windows installation.

### Tasks (abbreviated)

```
- [ ] **14.1 [SETUP]** Add windows-latest to CI matrix. Fix any path-separator issues in tests.
  Run `cargo test --workspace` on CI — must pass on all 3 OS.
- [ ] **14.2 [SETUP]** Add x86_64-pc-windows-msvc to release.yml. Handle .exe extension and
  .zip packaging. Add conditional strip step for non-Windows targets.
- [ ] **14.3 [SETUP]** Test Windows binary: download artifact from Actions, verify `ezpdf --version`.
- [ ] **14.4 [REVIEW]** CI green on windows-latest. Release has Windows artifact. Update README
  with Windows install instructions (cargo install / direct download). Commit. Progress update.
```

---

## Phase 15: Encrypted PDF Support

**Target:** v2
**Backlog:** B2 — "Encrypted PDF support (`--password` flag)"

### Definition of Done

- [ ] `--password` flag available on all 5 commands
- [ ] Operations succeed on password-protected PDFs when correct password is provided
- [ ] Wrong password → clear error message
- [ ] No password on encrypted PDF → existing "password-protected" error still shown

### Key Technical Notes

- **lopdf support:** lopdf 0.31 has limited decryption. Check `Document::load_with_password` or `doc.decrypt(password)`. May need to upgrade lopdf or use the `rc4` crate for RC4 decryption of 40/128-bit encrypted PDFs. AES-256 (PDF 2.0) is unlikely to be supported.
- **Alternative:** Shell out to `qpdf --decrypt --password=<pw> input.pdf temp.pdf` then process `temp.pdf`. This is simpler but adds a `qpdf` runtime dependency.
- **Research item:** Before implementation, check lopdf 0.31 changelog and issues for password decryption status.
- **`load_doc` update:** Add `pub fn load_doc_with_password(path: &Path, password: Option<&str>) -> Result<Document, EzPdfError>` variant.

### Tasks (abbreviated)

```
- [ ] **15.1 [RED]** Failing tests: encrypted PDF + correct password → operation succeeds;
  encrypted PDF + wrong password → PasswordRequired error; encrypted PDF + no password → EncryptedPdf error.
- [ ] **15.2 [GREEN]** Update EzPdfError with WrongPassword variant. Implement load_doc_with_password.
  Wire into all 5 commands. Handle lopdf decryption or qpdf fallback.
- [ ] **15.3 [RED]** Failing CLI tests for --password flag on all 5 commands.
- [ ] **15.4 [GREEN]** Add --password Option<String> to all Args structs. Pass through to core.
- [ ] **15.5 [REFACTOR]** --password-file flag (read password from file, useful for scripts).
- [ ] **15.6 [REVIEW]** DoD check. Commit. Progress update.
```

---

## Phase 16: Watermark Pages

**Target:** v2
**Backlog:** B9 — "Watermark pages (text or image)"

### Definition of Done

- [ ] `ezpdf watermark input.pdf "CONFIDENTIAL" -o output.pdf` adds a diagonal text watermark
- [ ] `--opacity`, `--color`, `--pages`, `--font-size` flags available
- [ ] Watermark is visually legible in Preview / Acrobat

### Key Technical Notes

- **Scope note:** Watermarking modifies page content streams — this breaks the "lossless" guarantee for page rendering. It is expected and intentional for this command.
- **lopdf approach:** For each target page, append a new `Stream` object to the page's `/Contents` array. The stream contains PDF graphics operators: `q` (save state), `cm` (matrix transform for rotation), `BT`/`ET` (begin/end text), `Tf` (font), `Tj` (draw text), `Q` (restore state). Need to add a `/Font` resource reference to the page's `/Resources` dictionary.
- **Font:** Use a standard PDF Type1 font (e.g., `/Helvetica`) — no embedding needed for standard 14 fonts.
- **Coordinates:** Page origin is bottom-left. For center-diagonal watermark: translate to center, rotate 45°.
- **Image watermark (stretch goal):** Embed image as `/XObject` of subtype `/Image`, reference from `/Resources`, draw with `Do` operator.

### Tasks (abbreviated)

```
- [ ] **16.1 [RED]** Failing tests: watermark text appears in page contents stream;
  page count unchanged after watermark.
- [ ] **16.2 [GREEN]** Create `ezpdf-core/src/watermark.rs`. Build PDF content stream string
  for text watermark. Append to each page's /Contents. Add Helvetica font resource.
- [ ] **16.3 [RED/GREEN]** CLI `ezpdf watermark` subcommand.
- [ ] **16.4 [REFACTOR]** --opacity (use graphics state alpha), --color (RGB), --angle.
- [ ] **16.5 [REVIEW]** Visual verification in Preview. Commit. Progress update.
```

---

## Phase 17: Bookmarks / Outline Manipulation

**Target:** v2
**Backlog:** B10 — "Bookmarks / outline manipulation"

### Definition of Done

- [ ] `ezpdf bookmarks list input.pdf` prints the outline tree (indented hierarchy)
- [ ] `ezpdf bookmarks add input.pdf --title "Chapter 1" --page 1 -o output.pdf` adds an entry
- [ ] `ezpdf bookmarks remove input.pdf --index 0 -o output.pdf` removes an entry

### Key Technical Notes

- **lopdf structure:** Outline is at `doc.catalog()` → `/Outlines` → root outline dict → linked list of `/First`, `/Next`, `/Last` dicts. Each entry has `/Title` (string), `/Dest` (page ref + coordinates or name), `/Count` (positive = open, negative = closed, 0 = leaf).
- **Traversal:** Recursive depth-first walk of `/First` → `/Next` chain.
- **Adding entries:** Create a new outline item dictionary, link it into the chain by updating `/Prev`/`/Next` of neighbors and `/Last` of parent.
- **`/Dest` format:** `[page_object_id 0 R /XYZ null null null]` for "top of page" destination.

### Tasks (abbreviated)

```
- [ ] **17.1 [RED]** Failing tests: list bookmarks (count, titles); add bookmark then list returns new entry.
- [ ] **17.2 [GREEN]** `ezpdf-core/src/bookmarks.rs` — list + add operations.
- [ ] **17.3 [RED/GREEN]** CLI `ezpdf bookmarks` subcommand with list/add/remove sub-subcommands.
- [ ] **17.4 [REFACTOR]** --format tree|flat|json for list output.
- [ ] **17.5 [REVIEW]** Verify in PDF reader. Commit. Progress update.
```

---

## Phase 18: Image Extraction

**Target:** v2
**Backlog:** B7 — "Extract images from PDF"

### Definition of Done

- [ ] `ezpdf images input.pdf -o ./images/` extracts all embedded images
- [ ] Images saved as `.jpg` (DCT-encoded) or `.png` (other filters decoded with deflate)
- [ ] Files named `page-{N}-image-{M}.jpg` / `.png`
- [ ] `--pages` flag to extract from specific pages only

### Key Technical Notes

- **lopdf approach:** For each page, read `/Resources` → `/XObject` dictionary. For each value where the referenced object has `/Subtype = /Image`: read the stream bytes via `doc.get_object(id)?.as_stream()?`, check `/Filter` (`/DCTDecode` → JPEG, `/FlateDecode` → PNG-style raw, `/JPXDecode` → JPEG 2000), read `/Width`, `/Height`, `/ColorSpace`.
- **For DCTDecode (JPEG):** stream bytes can be written directly as `.jpg`.
- **For FlateDecode:** decompress with `flate2` crate, then write raw pixel data as PNG using `png` crate (need to add dependencies).
- **Dependency additions:** `flate2 = "1"`, `png = "0.17"` in `ezpdf-core/Cargo.toml`.

### Tasks (abbreviated)

```
- [ ] **18.1 [RED]** Failing tests: extract images from a fixture PDF with known embedded image count.
- [ ] **18.2 [GREEN]** `ezpdf-core/src/images.rs` with `extract_images(input, output_dir)`.
- [ ] **18.3 [RED/GREEN]** CLI `ezpdf images` subcommand.
- [ ] **18.4 [REFACTOR]** --min-width/--min-height to filter out tiny icons/logos.
- [ ] **18.5 [REVIEW]** Verify extracted images are valid. Commit. Progress update.
```

---

## Phase 19: PDF Linearization / Optimization

**Target:** v2
**Backlog:** B8 — "PDF linearization / web optimization"

### Definition of Done

- [ ] `ezpdf optimize input.pdf -o output.pdf` produces a smaller or equally-sized PDF
- [ ] Or: `--linearize` mode rearranges objects for web fast-open

### Key Technical Notes

- **Scope clarification:** Full PDF linearization (per PDF spec section 8.6) requires precise object ordering with a linearization dictionary, hint tables, and cross-reference stream. This is non-trivial and lopdf may not support it.
- **Realistic scope for v2:**
  - **Object compaction:** Remove unreferenced objects (`doc.delete_unused_objects()`), remove orphaned pages.
  - **Stream recompression:** Re-deflate streams with higher compression (only valid for FlateDecode streams, keeping data identical — acceptable since stream data is preserved).
  - **Linearization:** Shell out to `qpdf --linearize` if qpdf is available; otherwise skip with a warning.
- **Research task 19.0:** Check if lopdf 0.31 has `delete_unused_objects` or similar. Check if there's a Rust crate for PDF linearization.

### Tasks (abbreviated)

```
- [ ] **19.0 [SETUP]** Research lopdf API for object cleanup. Determine if linearization is in scope
  or deferred to qpdf integration.
- [ ] **19.1 [RED]** Failing tests: optimize produces output that is valid PDF;
  optimize removes unreferenced objects (output size ≤ input size for a test fixture with junk objects).
- [ ] **19.2 [GREEN]** `ezpdf-core/src/optimize.rs` — object cleanup + optional stream recompression.
- [ ] **19.3 [RED/GREEN]** CLI `ezpdf optimize` subcommand with --linearize flag.
- [ ] **19.4 [REVIEW]** Verify output opens in PDF readers. Commit. Progress update.
```

---

## Phase 20: Desktop App (ezpdf-app)

**Target:** v2
**Backlog:** B3 — "ezpdf-app desktop app (Tauri v2 + Svelte 5)"

### Definition of Done

- [ ] `ezpdf-app` builds and runs on macOS (arm64 + x86_64)
- [ ] All 5 core operations available via GUI with file picker
- [ ] Drag-and-drop PDF input supported
- [ ] Packaged as `.app` bundle via Tauri

### Key Technical Notes

- **Scope:** This phase is intentionally coarse — the desktop app warrants its own brainstorm and plan. This phase adds the initial Tauri v2 + Svelte 5 scaffold and wires the first operation (merge) end-to-end as proof of concept.
- **Tauri v2:** Uses `@tauri-apps/api` v2. Rust commands are annotated with `#[tauri::command]`. Frontend calls via `invoke('merge', { inputs, output })`.
- **`ezpdf-app/src-tauri`**: Tauri Rust side — call `ezpdf_core` functions from Tauri command handlers.
- **`ezpdf-app/src`**: Svelte 5 frontend — file picker, operation cards, result display.
- **Dependencies to add:** `tauri = "2"` in `ezpdf-app/src-tauri/Cargo.toml`, `@tauri-apps/cli` and `@tauri-apps/api` in frontend.
- **Build:** `cargo tauri build` for release; `cargo tauri dev` for development.
- **Note:** A dedicated brainstorm (`docs/brainstorms/YYYY-MM-DD-ezpdf-app-brainstorm.md`) should be created before implementing Phase 20 in full.

### Tasks (abbreviated)

```
- [ ] **20.1 [SETUP]** Scaffold Tauri v2 + Svelte 5 inside ezpdf-app/. Run `cargo tauri init`.
  Verify `cargo tauri build` succeeds (even with placeholder UI).
- [ ] **20.2 [SETUP]** Add Tauri command `merge_pdfs(inputs: Vec<String>, output: String)` in
  `src-tauri/src/main.rs` that calls `ezpdf_core::merge`. Verify it runs via `invoke`.
- [ ] **20.3 [SETUP]** Build minimal Svelte 5 UI: file picker for inputs, output path selector,
  "Merge" button that calls the Tauri command, success/error toast.
- [ ] **20.4 [SETUP]** Add remaining 4 operations as Tauri commands + Svelte UI components.
- [ ] **20.5 [SETUP]** Add drag-and-drop file input support.
- [ ] **20.6 [REVIEW]** Manual end-to-end: drag PDFs, merge, verify output. Check DoD.
  Commit `feat: ezpdf-app desktop app scaffold (Phase 20)`. Update progress.md.
```

---

## Changes to ralph-loop.sh and PROMPT.md

Update the completion signal from `EZPDF V1 COMPLETE` to `EZPDF BACKLOG COMPLETE` for clarity:

**`ralph-loop.sh` line to change:**
```bash
# Before:
COMPLETION_SIGNAL="EZPDF V1 COMPLETE"

# After:
COMPLETION_SIGNAL="EZPDF BACKLOG COMPLETE"
```

**`PROMPT.md` completion section to change:**
```markdown
# Before:
When ALL tasks across ALL phases in `task_plan.md` are checked off (`- [x]`), output:
<promise>EZPDF V1 COMPLETE</promise>

# After:
When ALL tasks across ALL phases in `task_plan.md` are checked off (`- [x]`), output:
<promise>EZPDF BACKLOG COMPLETE</promise>
```

---

## Acceptance Criteria

- [ ] `task_plan.md` contains Phases 11–20 in the same format as Phases 1–10
- [ ] Old backlog table in `task_plan.md` is removed (items now live as phases)
- [ ] Each phase has a Definition of Done with unchecked boxes
- [ ] Each phase has the `> [!tip] Skills` note
- [ ] All tasks in phases 11–20 are unchecked (`- [ ]`)
- [ ] `ralph-loop.sh` and `PROMPT.md` completion signal are updated to `EZPDF BACKLOG COMPLETE`
- [ ] `progress.md` has a planning entry noting backlog phases 11–20 added
- [ ] `cargo test --workspace` still passes (no regressions from file edits)

## Sources & References

- Backlog items: `task_plan.md` (Backlog section, B1–B10)
- lopdf API: `ezpdf-core/src/` (existing implementations for patterns)
- Existing phase format: `task_plan.md` (Phases 1–10)
- ralph loop mechanics: `ralph-loop.sh`, `PROMPT.md`
