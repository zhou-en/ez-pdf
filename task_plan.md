# ezpdf v1 — Task Plan

> Last updated: 2026-03-12
> Status: 🟡 In progress

---

## Phase 1: Project Foundation

### Definition of Done

- [x] `cargo build --workspace` succeeds with zero warnings
- [x] `cargo test --workspace` passes (even with 0 tests)
- [x] `cargo clippy --workspace -- -D warnings` passes
- [x] `cargo fmt --check` passes
- [x] GitHub Actions CI runs on push (ubuntu-latest + macos-latest)

> [!tip] Skills for this phase
> No TDD skill needed — these are infrastructure tasks

### Tasks

- [x] **1.1 [SETUP]** Initialize git repo, create Cargo workspace with three crate stubs: `ezpdf-core` (lib), `ezpdf-cli` (bin), `ezpdf-app` (empty placeholder). Use workspace-level `Cargo.toml` with `resolver = "2"` and shared `[workspace.dependencies]` for `lopdf = "0.31"`, `thiserror = "2"`, `anyhow = "1"`, `clap = { version = "4", features = ["derive", "env"] }`, `clap_complete = "4"`, `clap_mangen = "0.2"`, `indicatif = "0.17"`, `rayon = "1"`, `criterion = { version = "0.5", features = ["html_reports"] }`. `ezpdf-cli/src/main.rs` prints `"ezpdf v0.1.0"` and exits.

- [x] **1.2 [SETUP]** Create `.github/workflows/ci.yml`: runs `cargo test --workspace`, `cargo clippy --workspace -- -D warnings`, `cargo fmt --check` on both `ubuntu-latest` and `macos-latest`. Use `dtolnay/rust-toolchain@stable` and `Swatinem/rust-cache@v2`.

- [x] **1.3 [SETUP]** Add `rustfmt.toml` (edition = "2021", max_width = 100), `clippy.toml` (msrv = "1.75"), `.gitignore` (Rust standard), `LICENSE` (MIT, author = "EZ"), `README.md` skeleton (name, one-line description, install + usage TBD), `CHANGELOG.md` (`[Unreleased]` section only).

- [x] **1.4 [REVIEW]** Run `cargo build --workspace && cargo clippy --workspace -- -D warnings && cargo fmt --check`. All must pass. Verify CI yaml is syntactically valid (`cat .github/workflows/ci.yml`). Check all Phase 1 DoD boxes. Commit all files. Update `progress.md`.

---

## Phase 2: Page Range Parser

### Definition of Done

- [x] `cargo test -p ezpdf-core` passes with 0 failures
- [x] All page range edge cases covered (see task 2.1 for the full list)
- [x] `EzPdfError` enum defined with `thiserror`

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **2.1 [RED]** Write failing tests for `ezpdf_core::page_range::parse(input: &str, page_count: u32) -> Result<Vec<u32>, EzPdfError>`. Cover: single page `"3"`, range `"1-5"`, list `"1,3,5"`, combined `"1-3,5,7-9"`, open-ended `"3-"` (to last page), all pages `"1-"`. Error cases: out of range (`"15"` for 10-page doc), invalid syntax (`"abc"`), reversed range (`"7-3"`), zero page (`"0"`), empty string. Use table-driven tests with a `cases` vec of `(input, page_count, expected_result)`. Also write unit tests for `EzPdfError` display messages — verify they contain the page count in out-of-range errors. Run `cargo test -p ezpdf-core` — tests must **FAIL** (items don't exist yet).

- [x] **2.2 [GREEN]** Create `ezpdf-core/src/error.rs` with `EzPdfError` enum (variants: `PageOutOfRange { page, total }`, `InvalidSyntax { input, hint }`, `EncryptedPdf`, `Io(#[from] std::io::Error)`, `Pdf(String)`). Create `ezpdf-core/src/page_range.rs` with `pub fn parse(input: &str, page_count: u32) -> Result<Vec<u32>, EzPdfError>`. Update `ezpdf-core/src/lib.rs` to export both modules. Run `cargo test -p ezpdf-core` — all tests must **PASS**.

- [x] **2.3 [REFACTOR]** Review `page_range.rs`: extract helper functions if the parse function is >40 lines. Ensure all `EzPdfError` messages are user-friendly. Run `cargo test -p ezpdf-core` — all tests still pass. Run `cargo clippy -p ezpdf-core -- -D warnings`.

- [x] **2.4 [REVIEW]** Check Phase 2 DoD. Run `cargo test -p ezpdf-core --verbose`. Count tests: should be ≥15 test cases. Commit. Update `progress.md`.

---

## Phase 3: Merge Command

### Definition of Done

- [x] `cargo test -p ezpdf-core merge` passes
- [x] `cargo test -p ezpdf-cli` passes (CLI integration tests)
- [x] Manual demo: `cargo run -p ezpdf-cli -- merge a.pdf b.pdf -o /tmp/out.pdf` produces a valid PDF with `a_pages + b_pages` pages

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **3.1 [SETUP]** Create PDF test fixture generator: `ezpdf-core/tests/common/mod.rs` with `pub fn create_test_pdf(page_count: u32, label: &str) -> Vec<u8>` that uses `lopdf` to create a minimal valid PDF with the given number of blank pages. Create fixture files: `ezpdf-core/tests/fixtures/3page.pdf` and `ezpdf-core/tests/fixtures/5page.pdf` by running the generator (write a small `build.rs` or use `cargo test -- --ignored` to generate once).

- [x] **3.2 [RED]** Write failing tests for `ezpdf_core::merge(inputs: &[&std::path::Path], output: &std::path::Path) -> Result<(), EzPdfError>`. Tests: merge 2 PDFs → output page count = sum of inputs; merge 3 PDFs → correct total; first input not found → `Io` error with the missing path; output to non-existent directory → `Io` error; encrypted PDF input → `EzPdfError::EncryptedPdf`. Use `tempfile` crate for output paths. Run `cargo test -p ezpdf-core` — new tests must **FAIL**.

- [x] **3.3 [GREEN]** Create `ezpdf-core/src/merge.rs`. Add `pub fn merge(inputs: &[&std::path::Path], output: &std::path::Path) -> Result<(), EzPdfError>` using `lopdf` to copy pages from each input document into a new target document, remapping object IDs. Detect encrypted PDFs via `lopdf::Document::load` error or `doc.is_encrypted()`. Export from `lib.rs`. Run `cargo test -p ezpdf-core` — all tests must **PASS**.

- [x] **3.4 [RED]** Write failing CLI integration test for `ezpdf merge` subcommand. Use `assert_cmd` crate: run `ezpdf merge fixture_a.pdf fixture_b.pdf -o /tmp/out.pdf`, assert exit code 0 and output contains "Merged". Test error: `ezpdf merge nonexistent.pdf -o /tmp/out.pdf` exits with code 1 and stderr contains "Error:". Run tests — must **FAIL** (no subcommand implemented yet).

- [x] **3.5 [GREEN]** Create `ezpdf-cli/src/commands/merge.rs` with clap derive struct `MergeArgs { files: Vec<PathBuf>, output: PathBuf, verbose: bool }`. Wire it into `ezpdf-cli/src/main.rs` as a subcommand. Add `assert_cmd` and `tempfile` to `ezpdf-cli/Cargo.toml` dev-dependencies. Run all tests — must **PASS**.

- [x] **3.6 [REFACTOR]** Extract a common `print_success(msg: &str, quiet: bool)` helper in CLI. Add `--quiet` global flag to suppress output. Run all tests still pass.

- [x] **3.7 [REVIEW]** Run `cargo test --workspace`. Run manual demo. Check Phase 3 DoD. Commit. Update `progress.md`.

---

## Phase 4: Remove Command

### Definition of Done

- [x] `cargo test -p ezpdf-core remove` passes
- [x] `cargo test -p ezpdf-cli remove` passes
- [x] Manual demo: remove pages from a 10-page PDF, open result, verify correct pages gone

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **4.1 [RED]** Write failing tests for `ezpdf_core::remove(input: &Path, pages: &str, output: &Path) -> Result<(), EzPdfError>`. Tests: remove middle page from 5-page PDF → output has 4 pages; remove first and last from 10-page PDF → 8 pages; remove all pages → `EzPdfError` (cannot remove all pages); page out of range → `EzPdfError::PageOutOfRange`. Run — must **FAIL**.

- [x] **4.2 [GREEN]** Create `ezpdf-core/src/remove.rs`. Implement by computing "pages to keep" = all pages minus removed pages, then using a split-like approach to copy kept pages to output. Export from `lib.rs`. All tests must **PASS**.

- [x] **4.3 [RED]** Write failing CLI tests for `ezpdf remove input.pdf 3,5 -o output.pdf`. Run — must **FAIL**.

- [x] **4.4 [GREEN]** Create `ezpdf-cli/src/commands/remove.rs`. Run all tests — must **PASS**.

- [x] **4.5 [REFACTOR]** Review error messages. Ensure "cannot remove all pages" includes page count context. Clippy clean.

- [x] **4.6 [REVIEW]** Run `cargo test --workspace`. Manual demo with 10-page fixture. Check DoD. Commit. Update `progress.md`.

---

## Phase 5: Split Command

### Definition of Done

- [x] `cargo test -p ezpdf-core split` passes
- [x] `cargo test -p ezpdf-cli split` passes
- [x] Demo: `split --each` produces N files named `page-001.pdf` through `page-00N.pdf`

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **5.1 [RED]** Write failing tests for `ezpdf_core::split_range` and `ezpdf_core::split_each`. `split_range` tests: extract pages 2-4 from 5-page PDF → output has 3 pages. `split_each` tests: burst 5-page PDF → 5 files created, named `page-1.pdf` through `page-5.pdf` (zero-padded to match digit count), output dir created if missing. Run — must **FAIL**.

- [x] **5.2 [GREEN]** Create `ezpdf-core/src/split.rs`. Implement both functions. Zero-pad filenames based on total page count (e.g., 10 pages → `page-01.pdf`; 100 pages → `page-001.pdf`). All tests must **PASS**.

- [x] **5.3 [RED]** Write failing CLI tests: mode 1 `split input.pdf 1-3 -o part.pdf`; mode 2 `split input.pdf --each -o /tmp/pages/`. Run — must **FAIL**.

- [x] **5.4 [GREEN]** Create `ezpdf-cli/src/commands/split.rs`. Handle both modes with `#[clap(group = ...)]` or subcommand branching. All tests must **PASS**.

- [x] **5.5 [REFACTOR]** Clean up. Ensure output directory creation is handled cleanly with good error messages if creation fails.

- [x] **5.6 [REVIEW]** Run `cargo test --workspace`. Manual demo: burst a PDF, verify filenames. Check DoD. Commit. Update `progress.md`.

---

## Phase 6: Rotate Command

### Definition of Done

- [x] `cargo test -p ezpdf-core rotate` passes
- [x] Round-trip test: rotate +90° then -90° equals original (verified by page /Rotate value)
- [x] Manual demo: open rotated PDF in Preview — pages visually rotated

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **6.1 [RED]** Write failing tests for `ezpdf_core::rotate(input: &Path, degrees: i32, pages: Option<&str>, output: &Path) -> Result<(), EzPdfError>`. Tests: rotate all pages 90° → each page dict has `/Rotate = 90`; rotate specific pages → only those pages have updated `/Rotate`; rotate by -90 → same as 270; invalid degrees (45) → `EzPdfError`; round-trip (rotate 90 then rotate -90) → `/Rotate` value equals original. Run — must **FAIL**.

- [x] **6.2 [GREEN]** Create `ezpdf-core/src/rotate.rs`. Implement by getting each target page's `ObjectId` via `doc.get_pages()`, then using `doc.get_object_mut(page_id)` to access the page `Dictionary` and call `dict.set("Rotate", Object::Integer(new_degrees))`. Read the existing `/Rotate` value first (default 0 if absent) and add the requested degrees mod 360. **Note: lopdf has no built-in `rotate_pages` API — you must edit the page dictionary directly.** All tests must **PASS**.

- [x] **6.3 [RED]** Write failing CLI tests for `ezpdf rotate input.pdf 90 -o out.pdf` and `ezpdf rotate input.pdf 90 --pages 1,3 -o out.pdf`. Run — must **FAIL**.

- [x] **6.4 [GREEN]** Create `ezpdf-cli/src/commands/rotate.rs`. All tests must **PASS**.

- [x] **6.5 [REFACTOR]** Clean up rotation logic. Normalize degrees to 0/90/180/270 before storing.

- [x] **6.6 [REVIEW]** Run `cargo test --workspace`. Manual demo in Preview. Check DoD. Commit. Update `progress.md`.

---

## Phase 7: Reorder Command

### Definition of Done

- [x] `cargo test -p ezpdf-core reorder` passes
- [x] Binary round-trip test: reorder then reorder back → identical to original
- [x] Manual demo: reorder pages, open in Preview, verify order

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **7.1 [RED]** Write failing tests for `ezpdf_core::reorder(input: &Path, order: &str, output: &Path) -> Result<(), EzPdfError>`. Tests: reorder `"3,1,2"` on 3-page PDF → page order changes correctly; round-trip `"2,1"` then `"2,1"` → same as original; missing page → `EzPdfError`; duplicate page → `EzPdfError`; order longer than page count → `EzPdfError`. Run — must **FAIL**.

- [x] **7.2 [GREEN]** Create `ezpdf-core/src/reorder.rs`. Parse the order string, get the root `/Pages` dictionary via `doc.get_pages_mut()` or traverse `doc.catalog()`, reorder the `/Kids` array to reflect the new order, and update the `/Count` integer to match. **Gotcha: if `/Count` is wrong or any page's `/Parent` ref is stale after reordering, lopdf will produce a structurally invalid PDF — verify the page tree is consistent before saving.** All tests must **PASS**.

- [x] **7.3 [RED]** Write failing CLI tests. Run — must **FAIL**.

- [x] **7.4 [GREEN]** Create `ezpdf-cli/src/commands/reorder.rs`. All tests must **PASS**.

- [x] **7.5 [REFACTOR]** Ensure `reorder` validation error messages explicitly name the missing/duplicate page.

- [x] **7.6 [REVIEW]** Run `cargo test --workspace`. Manual round-trip demo. Check DoD. Commit. Update `progress.md`.

> **🏁 After 7.6: all 5 core operations complete. ezpdf works end-to-end.**

---

## Phase 8: CLI Polish

### Definition of Done

- [x] `ezpdf --help` and `ezpdf <cmd> --help` show complete, example-rich help
- [x] `ezpdf completions zsh` produces valid zsh completion script
- [x] `ezpdf completions bash` and `ezpdf completions fish` work
- [x] Progress bar appears for operations on PDFs > 20 pages
- [x] Encrypted PDF detection works on all 5 commands
- [x] All error messages are user-friendly with recovery hints

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **8.1 [RED]** Write failing tests for: `ezpdf completions zsh` exits 0 and stdout is non-empty; `ezpdf completions bash` exits 0; `ezpdf --version` exits 0 and output contains "0.1.0"; `ezpdf nonexistent_command` exits 1 and stderr contains "error:". Run — must **FAIL**.

- [x] **8.2 [GREEN]** Add `completions` subcommand to CLI using `clap_complete`. Add `--version` flag. Add man page generation command `ezpdf man` via `clap_mangen`. Wire encrypted-PDF detection into all 5 command handlers (call a shared `check_not_encrypted(path)` helper that returns `EzPdfError::EncryptedPdf` with a `qpdf --decrypt` tip). All tests must **PASS**.

- [x] **8.3 [SETUP]** Add progress bar support using `indicatif` to merge, split-each, and remove commands. Threshold: show bar when PDF has > 20 pages. Bar format: `[████░░░░] Processing page 12/50 — merge`. Suppress with `--quiet`. (No unit tests for progress bar rendering — visual verification only.)

- [x] **8.4 [RED]** Write tests for all 5 commands that: (a) pass an encrypted PDF fixture → exit 1, stderr contains "password-protected" and "qpdf"; (b) pass a non-existent file → exit 1, stderr contains the file path. Run — must **FAIL**. (Need a minimal encrypted PDF fixture — create using `qpdf --encrypt "" "" 128 -- plain.pdf encrypted.pdf` in test setup, or embed a known-encrypted PDF byte sequence as a fixture.)

- [x] **8.5 [GREEN]** Implement the `check_not_encrypted` helper in `ezpdf-core`. Wire into all command handlers. Update all help strings to include `Examples:` sections. All tests must **PASS**.

- [x] **8.6 [REFACTOR]** Audit all `clap` `about`, `long_about`, `help` strings — every flag must have a description. Every subcommand must have 2+ examples in `long_about`. Run clippy clean.

- [x] **8.7 [REVIEW]** Run `cargo test --workspace`. Run all help commands manually. Pipe completions to a temp file and verify format. Check DoD. Commit. Update `progress.md`.

---

## Phase 9: Performance & Benchmarks

### Definition of Done

- [x] `cargo bench` runs without errors
- [x] Benchmark baseline committed to `docs/benchmarks/baseline.md`
- [x] Parallel merge shows measurable speedup for ≥3 inputs vs sequential

> [!tip] Skills for this phase
> - No TDD skill needed — benchmarks are additive, not feature development
> - Do not change any existing logic, only add benchmarks and parallelism

### Tasks

- [x] **9.1 [SETUP]** Create `ezpdf-core/benches/operations.rs` using `criterion`. Write benchmarks for: `merge` (5 × 10-page PDFs), `split_each` (50-page PDF), `remove` (remove half of 50-page PDF), `rotate` (all pages of 50-page PDF). Add large fixtures generation helper. Run `cargo bench -- --save-baseline baseline`. Commit benchmark results.

- [x] **9.2 [SETUP]** Add `rayon` to `ezpdf-core` dependencies. Implement parallel file loading in `merge.rs`: load and parse all input `Document`s in parallel using `rayon::iter`, then combine sequentially. Run `cargo bench -- --baseline baseline` to verify merge is not slower (should be faster for ≥3 files).

- [x] **9.3 [REVIEW]** Copy benchmark output to `docs/benchmarks/baseline.md`. Run `cargo test --workspace` — no regressions. Commit. Update `progress.md`.

---

## Phase 10: Distribution & Release

### Definition of Done

- [x] `brew install ez/tap/ezpdf` works on macOS Apple Silicon
- [x] GitHub Release v0.1.0 has 4 binary artifacts
- [x] `cargo install ezpdf` installs the binary

> [!tip] Skills for this phase
> - No TDD skill needed — infrastructure/release tasks

### Tasks

- [x] **10.1 [SETUP]** Create `.github/workflows/release.yml`. Matrix: `aarch64-apple-darwin` (macos-14), `x86_64-apple-darwin` (macos-13), `x86_64-unknown-linux-gnu` (ubuntu-22.04), `aarch64-unknown-linux-gnu` (ubuntu-22.04 + `cross`). Install cross via `taiki-e/install-action@cross` (not `cargo install cross`). Steps: checkout, `dtolnay/rust-toolchain@stable`, install cross if needed, `cargo build --release --target <target>`, strip binary, upload artifact. Final job: create GitHub release with `softprops/action-gh-release@v2` and attach all artifacts. **Note: use ubuntu-22.04 specifically — Tauri v2 desktop app (v2) requires `libwebkit2gtk-4.1` which is not available on ubuntu-20.04.**

- [x] **10.2 [SETUP]** Push a `v0.1.0` tag to trigger the release workflow: `git tag v0.1.0 && git push origin v0.1.0`. Monitor the Actions workflow. Once release exists, note the download URLs for each binary.

- [x] **10.3 [SETUP]** Create Homebrew tap formula. If `github.com/ez/homebrew-tap` doesn't exist yet, create the repo via `gh repo create ez/homebrew-tap --public`. Write `Formula/ezpdf.rb` with `url`, `sha256`, `version` for each platform. Install man page via `man1.install`. Install shell completions. Test locally: `brew install --formula ./Formula/ezpdf.rb`.

- [x] **10.4 [SETUP]** Publish to crates.io: ensure `ezpdf-core/Cargo.toml` and `ezpdf-cli/Cargo.toml` have `description`, `repository`, `keywords`, `categories`. Run `cargo publish -p ezpdf-core`, then `cargo publish -p ezpdf-cli`.

- [x] **10.5 [SETUP]** Update `README.md` with: installation (Homebrew + cargo install), full usage examples for all 5 commands, link to man page, link to GitHub releases.

- [x] **10.6 [REVIEW]** End-to-end install test: `brew install ez/tap/ezpdf`, run all 5 commands with real PDFs, verify correct output. Run `cargo install ezpdf` in a clean environment. Update `progress.md`. **v1.0.0 is RELEASED.**

---

## Blockers

_None yet. Blockers found during stories will be injected here._

---

## Phase 11: `ezpdf info` Command

### Definition of Done

- [x] `cargo test -p ezpdf-core info` passes
- [x] `cargo test -p ezpdf-cli info` passes
- [x] `ezpdf info input.pdf` prints page count, per-page dimensions, and document metadata fields
- [x] `ezpdf info input.pdf --json` outputs valid JSON

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **11.1 [RED]** Write failing tests for `ezpdf_core::info(input: &Path) -> Result<PdfInfo, EzPdfError>`.
  Tests: (a) info on 3page.pdf → `page_count == 3`; (b) dimensions vec has 3 entries, each > 0.0;
  (c) info on nonexistent file → `Io` error; (d) info on encrypted.pdf fixture → `EncryptedPdf` error.
  Also define `PdfInfo` struct in the test file (it won't exist yet) with fields:
  `page_count: u32`, `dimensions: Vec<(f64, f64)>`, `title: Option<String>`, `author: Option<String>`,
  `subject: Option<String>`, `keywords: Option<String>`, `creator: Option<String>`, `producer: Option<String>`.
  Run `cargo test -p ezpdf-core` — tests must **FAIL**.

- [x] **11.2 [GREEN]** Create `ezpdf-core/src/info.rs`. Define `PdfInfo` struct (derive Debug, PartialEq).
  Implement `pub fn info(input: &Path) -> Result<PdfInfo, EzPdfError>`.
  Use `load_doc(input)?` for loading and encrypted detection.
  Extract page count via `doc.get_pages().len() as u32`.
  Extract dimensions: iterate `doc.get_pages()` sorted by page number; for each page object id,
  call `doc.get_object(id)?.as_dict()?`, look for `/MediaBox` array `[x0, y0, x1, y1]` (may be
  inherited from parent /Pages dict — walk up if absent); compute `width = x1 - x0`, `height = y1 - y0`.
  Extract metadata: follow `doc.trailer.get(b"Info")` reference to the Info dictionary,
  read each key as `Option<String>` (handle PDFDocEncoding byte strings and UTF-16BE with BOM `\xFE\xFF`).
  Export from `lib.rs`. All tests must **PASS**.

- [x] **11.3 [RED]** Write failing CLI tests for `ezpdf info`:
  (a) `ezpdf info 3page.pdf` exits 0, stdout contains "Pages: 3";
  (b) `ezpdf info 3page.pdf --json` exits 0, stdout is parseable JSON with `page_count` field;
  (c) `ezpdf info nonexistent.pdf` exits 1, stderr contains "Error:".
  Run tests — must **FAIL**.

- [x] **11.4 [GREEN]** Create `ezpdf-cli/src/commands/info.rs` with `InfoArgs { file: PathBuf, json: bool }`.
  Wire as `ezpdf info` subcommand in `main.rs`. For normal output: print `File: {}`, `Pages: {}`,
  a dimensions table (Page | Width pt | Height pt), and metadata fields (skip None values).
  For `--json`: add `serde` feature to `ezpdf-core` (`serde = { version = "1", features = ["derive"] }`)
  and `serde_json = "1"` to `ezpdf-cli`; derive `Serialize` on `PdfInfo`; serialize and print.
  All tests must **PASS**.

- [x] **11.5 [REFACTOR]** In normal output mode, detect common paper sizes from dimensions
  (A4 = 595×842 pt, Letter = 612×792 pt, Legal = 612×1008 pt) within 2 pt tolerance and
  append the size name in parentheses. Add `--pages` flag to show dimensions for specific pages only
  (reuse `page_range::parse`). Clippy clean.

- [x] **11.6 [REVIEW]** Run `cargo test --workspace`. Manual demo: `ezpdf info` on a real PDF.
  Check Phase 11 DoD. Commit `feat: ezpdf info command (Phase 11)`. Update `progress.md`.

---

## Phase 12: Batch Operations

### Definition of Done

- [x] `--batch` flag works on `rotate`, `remove`, `reorder` (apply operation independently to each PDF in a directory)
- [x] `merge --batch DIR/ -o out.pdf` merges all PDFs in a directory into one output file
- [x] `split --batch DIR/ -o OUT_DIR/` splits each PDF into its own output subdirectory
- [x] Progress bar shown when batch contains > 1 file

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **12.1 [RED]** Write failing tests for `ezpdf_core::batch::collect_pdf_inputs(dir: &Path) -> Result<Vec<PathBuf>, EzPdfError>`.
  Tests: (a) dir with 3 `.pdf` files → returns 3 paths sorted alphabetically;
  (b) dir with mixed `.pdf` and `.txt` files → only `.pdf` included;
  (c) nonexistent dir → `Io` error; (d) empty dir → empty vec (not an error).
  Run `cargo test -p ezpdf-core` — must **FAIL**.

- [x] **12.2 [GREEN]** Create `ezpdf-core/src/batch.rs`. Implement `collect_pdf_inputs` using
  `std::fs::read_dir`, filtering entries where `.extension() == Some("pdf")`, sorting by filename.
  Export from `lib.rs`. All tests must **PASS**.

- [x] **12.3 [RED]** Write failing CLI tests for `--batch` on each command:
  (a) `ezpdf rotate --batch fixtures_dir/ 90 -o out_dir/` exits 0, output dir contains N PDFs with same names;
  (b) `ezpdf remove --batch fixtures_dir/ 1 -o out_dir/` exits 0;
  (c) `ezpdf merge --batch fixtures_dir/ -o out.pdf` exits 0, output is a single PDF;
  (d) `ezpdf rotate --batch nonexistent/ 90 -o out/` exits 1 with "Error:".
  Use `tempfile::tempdir()` for all output paths. Run — must **FAIL**.

- [x] **12.4 [GREEN]** Add `batch: bool` to all 5 command `Args` structs with `#[arg(long)]`.
  In each command handler: when `batch` is true, treat the input path as a directory,
  call `collect_pdf_inputs`, create the output directory with `std::fs::create_dir_all`,
  then loop over each input applying the operation. For `merge --batch`: collect all inputs,
  call `merge()` once with all of them. For `split --batch`: create a subdirectory per input file
  named after the stem. For other commands: apply independently, save to `out_dir/<original_filename>`.
  Show a `ProgressBar` (via `indicatif`) when the batch contains > 1 file. All tests must **PASS**.

- [x] **12.5 [REFACTOR]** Extract a `run_batch_independent<F>(inputs: Vec<PathBuf>, out_dir: &Path, quiet: bool, op: F)`
  helper in `ezpdf-cli/src/output.rs` to reduce duplication across the 4 independent-file commands.
  `F` has signature `Fn(&Path, &Path) -> Result<(), EzPdfError>`. Clippy clean.

- [x] **12.6 [REVIEW]** Run `cargo test --workspace`. Manual demo: batch rotate a directory of PDFs.
  Check Phase 12 DoD. Commit `feat: batch --batch flag (Phase 12)`. Update `progress.md`.

---

## Phase 13: PDF Metadata Read/Write

### Definition of Done

- [x] `ezpdf meta get input.pdf` prints all metadata fields present in the document
- [x] `ezpdf meta set input.pdf --title "..." -o output.pdf` updates selected fields and saves
- [x] Round-trip: `set` then `get` returns the values that were set
- [x] `ezpdf meta get input.pdf --json` outputs valid JSON

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **13.1 [RED]** Write failing tests for `ezpdf_core::get_metadata(input: &Path) -> Result<PdfMetadata, EzPdfError>`
  and `ezpdf_core::set_metadata(input: &Path, updates: MetadataUpdate, output: &Path) -> Result<(), EzPdfError>`.
  Define `PdfMetadata` struct (all fields `Option<String>`: title, author, subject, keywords, creator, producer)
  and `MetadataUpdate` struct (same optional fields plus `clear_all: bool`).
  Tests: (a) get_metadata on fixture → no error; (b) set title then get → title matches;
  (c) set multiple fields in one call → all updated; (d) `clear_all: true` wipes all fields.
  Run — must **FAIL**.

- [x] **13.2 [GREEN]** Create `ezpdf-core/src/metadata.rs`. Implement `get_metadata`: follow
  `doc.trailer.get(b"Info")` reference chain to the Info dictionary; read each field.
  Implement `set_metadata`: load doc, get or create the Info dictionary, update only `Some` fields,
  save with `doc.save(output)?`. Handle the case where no Info dict exists (create one, add to trailer).
  Export from `lib.rs`. All tests must **PASS**.

- [x] **13.3 [RED]** Failing CLI tests: `ezpdf meta get input.pdf` exits 0 and stdout contains field names;
  `ezpdf meta get input.pdf --json` exits 0 and stdout is valid JSON;
  `ezpdf meta set input.pdf --title "Test" -o out.pdf` exits 0;
  then `ezpdf meta get out.pdf` shows "Test". Run — must **FAIL**.

- [x] **13.4 [GREEN]** Create `ezpdf-cli/src/commands/meta.rs` with nested clap subcommands `get` and `set`.
  `GetArgs { file: PathBuf, json: bool }`. `SetArgs { file: PathBuf, output: PathBuf, title: Option<String>,
  author: Option<String>, subject: Option<String>, keywords: Option<String>, clear_all: bool }`.
  All tests must **PASS**.

- [x] **13.5 [REFACTOR]** For `meta get` normal output: print as aligned key: value pairs, skip None fields.
  Derive `Serialize` on `PdfMetadata` for JSON output. Clippy clean.

- [x] **13.6 [REVIEW]** Run `cargo test --workspace`. Round-trip demo. Check Phase 13 DoD.
  Commit `feat: ezpdf meta command (Phase 13)`. Update `progress.md`.

---

## Phase 15: Encrypted PDF Support

### Definition of Done

- [x] `--password` flag available on all 5 commands
- [x] Operations succeed on password-protected PDFs when the correct password is provided
- [x] Wrong password → clear `EzPdfError::WrongPassword` with recovery hint
- [x] No password on encrypted PDF → existing `EncryptedPdf` error unchanged

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **15.1 [SETUP]** Research lopdf 0.31 password decryption support. Check `Document::load_with_password`
  or `doc.decrypt(password)` in the lopdf source / changelog. If lopdf does not support decryption,
  implement a `qpdf --decrypt --password=<pw>` shell-out fallback using `std::process::Command`
  writing to a `tempfile`. Document the chosen approach in a code comment. Update `progress.md`
  with the finding before proceeding.

- [x] **15.2 [RED]** Add `WrongPassword` variant to `EzPdfError`. Write failing tests:
  (a) `load_doc_with_password(encrypted_fixture, Some("correct_pw"))` → Ok;
  (b) `load_doc_with_password(encrypted_fixture, Some("wrong_pw"))` → `WrongPassword` error;
  (c) `load_doc_with_password(encrypted_fixture, None)` → `EncryptedPdf` error (unchanged behaviour).
  **Note:** Need an encrypted fixture with a known password — create one in test setup using
  `qpdf --encrypt "secret" "secret" 128 -- plain.pdf encrypted_pw.pdf` (or embed hardcoded bytes).
  Run — must **FAIL**.

- [x] **15.3 [GREEN]** Implement `load_doc_with_password(path: &Path, password: Option<&str>) -> Result<Document, EzPdfError>`
  in `ezpdf-core/src/load.rs` (or `lib.rs`). Use the approach determined in 15.1.
  Update `check_not_encrypted` → `maybe_load_doc` to accept an optional password.
  All tests must **PASS**.

- [x] **15.4 [RED]** Failing CLI tests: `ezpdf merge --password secret encrypted_pw.pdf plain.pdf -o out.pdf` exits 0;
  `ezpdf rotate --password wrong encrypted_pw.pdf 90 -o out.pdf` exits 1, stderr contains "password".
  Run — must **FAIL**.

- [x] **15.5 [GREEN]** Add `--password Option<String>` to all 5 `Args` structs. Pass through to core.
  All tests must **PASS**.

- [x] **15.6 [REFACTOR]** Add `--password-file <path>` flag (reads password from a file, strips trailing newline).
  Clippy clean.

- [x] **15.7 [REVIEW]** Run `cargo test --workspace`. Demo: operate on a real encrypted PDF.
  Check Phase 15 DoD. Commit `feat: encrypted PDF --password support (Phase 15)`. Update `progress.md`.

---

## Phase 16: Watermark Pages

### Definition of Done

- [x] `ezpdf watermark input.pdf "CONFIDENTIAL" -o output.pdf` adds a diagonal text watermark
- [x] `--opacity`, `--color`, `--font-size`, `--pages` flags available
- [x] Watermark is visually legible when opened in Preview / Acrobat

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **16.1 [RED]** Write failing tests for `ezpdf_core::watermark(input: &Path, text: &str, opts: WatermarkOptions, output: &Path) -> Result<(), EzPdfError>`.
  Define `WatermarkOptions { opacity: f32, color_rgb: (f32, f32, f32), font_size: f32, pages: Option<String> }`.
  Tests: (a) watermark a 3-page PDF → output has 3 pages (page count unchanged);
  (b) output file contains the watermark text bytes somewhere in its binary content
  (look for the text string in the raw file bytes after watermarking).
  Run — must **FAIL**.

- [x] **16.2 [GREEN]** Create `ezpdf-core/src/watermark.rs`. For each target page:
  (1) Build a PDF content stream string with graphics operators:
      `q` (save state), set graphics state (`/ca <opacity> gs`), CTM translate to page center and
      rotate 45°, `BT /Helvetica <font_size> Tf <color> rg (<text>) Tj ET`, `Q` (restore state).
  (2) Create a new `Stream` object in the document with this content.
  (3) Append the new stream object id to the page's `/Contents` array (create array if it's currently a direct reference).
  (4) Add `/Font << /Helvetica <font_resource_ref> >>` to the page's `/Resources` dictionary,
      referencing a standard Type1 font object. All tests must **PASS**.
  **Note:** This intentionally modifies content streams. The lossless guarantee does not apply to watermark.

- [x] **16.3 [RED]** Failing CLI tests for `ezpdf watermark input.pdf "DRAFT" -o out.pdf` exits 0;
  `ezpdf watermark input.pdf "DRAFT" --pages 1,3 -o out.pdf` exits 0; wrong input exits 1. Run — must **FAIL**.

- [x] **16.4 [GREEN]** Create `ezpdf-cli/src/commands/watermark.rs`. All tests must **PASS**.

- [x] **16.5 [REFACTOR]** Center-align text horizontally using approximate character width estimates.
  Clippy clean.

- [x] **16.6 [REVIEW]** Visual verification: open watermarked PDF in Preview, confirm text is visible.
  Run `cargo test --workspace`. Check Phase 16 DoD.
  Commit `feat: watermark command (Phase 16)`. Update `progress.md`.

---

## Phase 17: Bookmarks / Outline Manipulation

### Definition of Done

- [x] `ezpdf bookmarks list input.pdf` prints the outline tree (indented to show hierarchy)
- [x] `ezpdf bookmarks add input.pdf --title "Chapter 1" --page 1 -o output.pdf` adds an entry
- [x] Round-trip: add then list shows the new entry

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **17.1 [RED]** Write failing tests for:
  `ezpdf_core::list_bookmarks(input: &Path) -> Result<Vec<Bookmark>, EzPdfError>` and
  `ezpdf_core::add_bookmark(input: &Path, title: &str, page: u32, output: &Path) -> Result<(), EzPdfError>`.
  Define `Bookmark { title: String, page: u32, level: u32 }`.
  Tests: (a) list on a PDF with no outline → empty vec; (b) add bookmark then list → entry present
  with correct title and page; (c) add two bookmarks → both present in order.
  Run — must **FAIL**.

- [x] **17.2 [GREEN]** Create `ezpdf-core/src/bookmarks.rs`.
  `list_bookmarks`: navigate `doc.catalog()` → `/Outlines` → follow `/First`→`/Next` chain recursively,
  collecting `Bookmark` entries. `/Dest` array gives the page object id — map back to page number using
  `doc.get_pages()` (which returns a `BTreeMap<u32, ObjectId>`).
  `add_bookmark`: create a new outline item dictionary with `/Title` (pdf string), `/Dest` array
  (`[page_obj_id 0 R /XYZ null null null]`), link into the outline chain by updating `/Last` of
  the root Outlines dict and `/Prev` of the previous last entry. If no `/Outlines` exists in the
  catalog, create one. All tests must **PASS**.

- [x] **17.3 [RED]** Failing CLI tests: `ezpdf bookmarks list input.pdf` exits 0;
  `ezpdf bookmarks add input.pdf --title "Ch1" --page 1 -o out.pdf` exits 0;
  then `ezpdf bookmarks list out.pdf` stdout contains "Ch1". Run — must **FAIL**.

- [x] **17.4 [GREEN]** Create `ezpdf-cli/src/commands/bookmarks.rs` with nested subcommands `list` and `add`.
  All tests must **PASS**.

- [x] **17.5 [REFACTOR]** `list` output: indent by `level` (2 spaces per level). Add `--json` flag.
  Clippy clean.

- [x] **17.6 [REVIEW]** Open bookmarked PDF in Preview, verify outline panel shows entries.
  Run `cargo test --workspace`. Check Phase 17 DoD.
  Commit `feat: bookmarks command (Phase 17)`. Update `progress.md`.

---

## Phase 18: Image Extraction

### Definition of Done

- [x] `ezpdf images input.pdf -o ./images/` extracts all embedded XObject images
- [x] JPEG images saved as `.jpg`, others decoded and saved as `.png`
- [x] Files named `page-{N}-image-{M}.jpg` / `.png`

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [x] **18.1 [SETUP]** Create a test fixture `ezpdf-core/tests/fixtures/with_image.pdf` containing at
  least one embedded JPEG XObject. Use lopdf to embed a tiny JPEG (grab any 1×1 pixel JPEG bytes)
  as an `/Image` XObject in a test PDF page. Commit the fixture.

- [x] **18.2 [RED]** Write failing tests for `ezpdf_core::extract_images(input: &Path, output_dir: &Path) -> Result<u32, EzPdfError>` (returns count of images extracted).
  Tests: (a) extract from `with_image.pdf` → count > 0, at least one file created in output_dir;
  (b) extract from `3page.pdf` (no images) → count == 0, no files created;
  (c) nonexistent input → Io error. Run — must **FAIL**.

- [x] **18.3 [GREEN]** Create `ezpdf-core/src/images.rs`. Add `flate2 = "1"` dependency.
  For each page in `doc.get_pages()`: get page `/Resources` → `/XObject` dictionary.
  For each value, follow the reference → check `/Subtype = /Image`. Get the stream object.
  Check `/Filter`: if `/DCTDecode` → write stream bytes directly as `.jpg`.
  If `/FlateDecode` → decompress with `flate2::read::ZlibDecoder`, read `/Width`, `/Height`,
  `/ColorSpace` (RGB=3 channels, Gray=1 channel), write raw pixels as PNG using the `png` crate
  (add `png = "0.17"` dependency). Name files `page-{N}-image-{M}.ext`.
  Create output dir if missing. Return total count. All tests must **PASS**.

- [x] **18.4 [RED]** Failing CLI tests: `ezpdf images with_image.pdf -o out_dir/` exits 0,
  stdout contains "Extracted"; `ezpdf images 3page.pdf -o out_dir/` exits 0, stdout contains "0 image".
  Run — must **FAIL**.

- [x] **18.5 [GREEN]** Create `ezpdf-cli/src/commands/images.rs`. Add `--pages` flag to limit extraction.
  Add `--min-width` / `--min-height` flags (default 10) to skip tiny decorative images.
  All tests must **PASS**.

- [x] **18.6 [REVIEW]** Run `cargo test --workspace`. Manual demo on a PDF with images.
  Check Phase 18 DoD. Commit `feat: image extraction command (Phase 18)`. Update `progress.md`.

---

## Phase 19: PDF Optimization

### Definition of Done

- [ ] `ezpdf optimize input.pdf -o output.pdf` produces a valid PDF with unreferenced objects removed
- [ ] Output file size is ≤ input file size for any input with unused objects
- [ ] `--linearize` flag attempts linearization (via qpdf if available, else skipped with warning)

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [ ] **19.0 [SETUP]** Check lopdf 0.31 for object cleanup API (search for `delete_unused`, `prune`,
  or similar in lopdf source). Document findings in a code comment in the new `optimize.rs`.
  If lopdf has no built-in cleanup, implement manual object reachability traversal.

- [ ] **19.1 [RED]** Create a test fixture `bloated.pdf` — a PDF with unreferenced objects injected
  (use lopdf to add orphaned dictionary objects not reachable from the catalog).
  Write failing tests for `ezpdf_core::optimize(input: &Path, output: &Path) -> Result<OptimizeStats, EzPdfError>`.
  Define `OptimizeStats { objects_removed: u32, bytes_saved: i64 }`.
  Tests: (a) optimize `bloated.pdf` → `objects_removed > 0`; (b) optimize `3page.pdf` → output is valid PDF;
  (c) output has same page count as input. Run — must **FAIL**.

- [ ] **19.2 [GREEN]** Create `ezpdf-core/src/optimize.rs`. Implement reachability traversal:
  start from the trailer, follow all object references recursively, collect the set of referenced
  object ids. Delete all objects NOT in the set using `doc.objects.retain(|id, _| reachable.contains(id))`.
  Re-save with `doc.save(output)?`. All tests must **PASS**.

- [ ] **19.3 [RED]** Failing CLI tests: `ezpdf optimize input.pdf -o out.pdf` exits 0, stdout contains "Optimized";
  `ezpdf optimize input.pdf --linearize -o out.pdf` exits 0 (linearize with qpdf or skip with warning).
  Run — must **FAIL**.

- [ ] **19.4 [GREEN]** Create `ezpdf-cli/src/commands/optimize.rs`. For `--linearize`: attempt
  `qpdf --linearize in.pdf out.pdf` via `std::process::Command`; if qpdf is not found, print a
  warning and fall back to normal optimize. All tests must **PASS**.

- [ ] **19.5 [REVIEW]** Run `cargo test --workspace`. Demo on a PDF, show bytes saved.
  Check Phase 19 DoD. Commit `feat: optimize command (Phase 19)`. Update `progress.md`.

---

## Phase 20: Desktop App (ezpdf-app)

> **TODO — Requires separate planning before ralph can implement this.**
>
> The desktop app (Tauri v2 + Svelte 5) requires a different toolchain (Node.js, npm, Tauri CLI)
> and substantial UI/UX design decisions that benefit from human input before automating.
>
> **Before activating this phase:**
> 1. Run `/ce:brainstorm` for the desktop app to produce `docs/brainstorms/YYYY-MM-DD-ezpdf-app-brainstorm.md`
> 2. Run `/ce:plan` to convert the brainstorm into detailed tasks in this phase
> 3. Replace this note with the resulting tasks, then run ralph
>
> Phase 20 tasks are intentionally left unchecked here so the completion signal fires after phases 11–19.

- [x] **20.0 [TODO]** Desktop app planning deferred — see note above. Phases 11–19 complete the CLI backlog.

---

## Phase 21: Windows Support

> **TODO — Deferred. Implement after the desktop app when Windows demand warrants it.**

- [x] **21.0 [TODO]** Windows support deferred — add to a future planning session.
