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
- [ ] GitHub Actions CI runs on push (ubuntu-latest + macos-latest)

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

- [ ] `cargo test -p ezpdf-core merge` passes
- [ ] `cargo test -p ezpdf-cli` passes (CLI integration tests)
- [ ] Manual demo: `cargo run -p ezpdf-cli -- merge a.pdf b.pdf -o /tmp/out.pdf` produces a valid PDF with `a_pages + b_pages` pages

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [ ] **3.1 [SETUP]** Create PDF test fixture generator: `ezpdf-core/tests/common/mod.rs` with `pub fn create_test_pdf(page_count: u32, label: &str) -> Vec<u8>` that uses `lopdf` to create a minimal valid PDF with the given number of blank pages. Create fixture files: `ezpdf-core/tests/fixtures/3page.pdf` and `ezpdf-core/tests/fixtures/5page.pdf` by running the generator (write a small `build.rs` or use `cargo test -- --ignored` to generate once).

- [ ] **3.2 [RED]** Write failing tests for `ezpdf_core::merge(inputs: &[&std::path::Path], output: &std::path::Path) -> Result<(), EzPdfError>`. Tests: merge 2 PDFs → output page count = sum of inputs; merge 3 PDFs → correct total; first input not found → `Io` error with the missing path; output to non-existent directory → `Io` error; encrypted PDF input → `EzPdfError::EncryptedPdf`. Use `tempfile` crate for output paths. Run `cargo test -p ezpdf-core` — new tests must **FAIL**.

- [ ] **3.3 [GREEN]** Create `ezpdf-core/src/merge.rs`. Add `pub fn merge(inputs: &[&std::path::Path], output: &std::path::Path) -> Result<(), EzPdfError>` using `lopdf` to copy pages from each input document into a new target document, remapping object IDs. Detect encrypted PDFs via `lopdf::Document::load` error or `doc.is_encrypted()`. Export from `lib.rs`. Run `cargo test -p ezpdf-core` — all tests must **PASS**.

- [ ] **3.4 [RED]** Write failing CLI integration test for `ezpdf merge` subcommand. Use `assert_cmd` crate: run `ezpdf merge fixture_a.pdf fixture_b.pdf -o /tmp/out.pdf`, assert exit code 0 and output contains "Merged". Test error: `ezpdf merge nonexistent.pdf -o /tmp/out.pdf` exits with code 1 and stderr contains "Error:". Run tests — must **FAIL** (no subcommand implemented yet).

- [ ] **3.5 [GREEN]** Create `ezpdf-cli/src/commands/merge.rs` with clap derive struct `MergeArgs { files: Vec<PathBuf>, output: PathBuf, verbose: bool }`. Wire it into `ezpdf-cli/src/main.rs` as a subcommand. Add `assert_cmd` and `tempfile` to `ezpdf-cli/Cargo.toml` dev-dependencies. Run all tests — must **PASS**.

- [ ] **3.6 [REFACTOR]** Extract a common `print_success(msg: &str, quiet: bool)` helper in CLI. Add `--quiet` global flag to suppress output. Run all tests still pass.

- [ ] **3.7 [REVIEW]** Run `cargo test --workspace`. Run manual demo. Check Phase 3 DoD. Commit. Update `progress.md`.

---

## Phase 4: Remove Command

### Definition of Done

- [ ] `cargo test -p ezpdf-core remove` passes
- [ ] `cargo test -p ezpdf-cli remove` passes
- [ ] Manual demo: remove pages from a 10-page PDF, open result, verify correct pages gone

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [ ] **4.1 [RED]** Write failing tests for `ezpdf_core::remove(input: &Path, pages: &str, output: &Path) -> Result<(), EzPdfError>`. Tests: remove middle page from 5-page PDF → output has 4 pages; remove first and last from 10-page PDF → 8 pages; remove all pages → `EzPdfError` (cannot remove all pages); page out of range → `EzPdfError::PageOutOfRange`. Run — must **FAIL**.

- [ ] **4.2 [GREEN]** Create `ezpdf-core/src/remove.rs`. Implement by computing "pages to keep" = all pages minus removed pages, then using a split-like approach to copy kept pages to output. Export from `lib.rs`. All tests must **PASS**.

- [ ] **4.3 [RED]** Write failing CLI tests for `ezpdf remove input.pdf 3,5 -o output.pdf`. Run — must **FAIL**.

- [ ] **4.4 [GREEN]** Create `ezpdf-cli/src/commands/remove.rs`. Run all tests — must **PASS**.

- [ ] **4.5 [REFACTOR]** Review error messages. Ensure "cannot remove all pages" includes page count context. Clippy clean.

- [ ] **4.6 [REVIEW]** Run `cargo test --workspace`. Manual demo with 10-page fixture. Check DoD. Commit. Update `progress.md`.

---

## Phase 5: Split Command

### Definition of Done

- [ ] `cargo test -p ezpdf-core split` passes
- [ ] `cargo test -p ezpdf-cli split` passes
- [ ] Demo: `split --each` produces N files named `page-001.pdf` through `page-00N.pdf`

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [ ] **5.1 [RED]** Write failing tests for `ezpdf_core::split_range` and `ezpdf_core::split_each`. `split_range` tests: extract pages 2-4 from 5-page PDF → output has 3 pages. `split_each` tests: burst 5-page PDF → 5 files created, named `page-1.pdf` through `page-5.pdf` (zero-padded to match digit count), output dir created if missing. Run — must **FAIL**.

- [ ] **5.2 [GREEN]** Create `ezpdf-core/src/split.rs`. Implement both functions. Zero-pad filenames based on total page count (e.g., 10 pages → `page-01.pdf`; 100 pages → `page-001.pdf`). All tests must **PASS**.

- [ ] **5.3 [RED]** Write failing CLI tests: mode 1 `split input.pdf 1-3 -o part.pdf`; mode 2 `split input.pdf --each -o /tmp/pages/`. Run — must **FAIL**.

- [ ] **5.4 [GREEN]** Create `ezpdf-cli/src/commands/split.rs`. Handle both modes with `#[clap(group = ...)]` or subcommand branching. All tests must **PASS**.

- [ ] **5.5 [REFACTOR]** Clean up. Ensure output directory creation is handled cleanly with good error messages if creation fails.

- [ ] **5.6 [REVIEW]** Run `cargo test --workspace`. Manual demo: burst a PDF, verify filenames. Check DoD. Commit. Update `progress.md`.

---

## Phase 6: Rotate Command

### Definition of Done

- [ ] `cargo test -p ezpdf-core rotate` passes
- [ ] Round-trip test: rotate +90° then -90° equals original (verified by page /Rotate value)
- [ ] Manual demo: open rotated PDF in Preview — pages visually rotated

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [ ] **6.1 [RED]** Write failing tests for `ezpdf_core::rotate(input: &Path, degrees: i32, pages: Option<&str>, output: &Path) -> Result<(), EzPdfError>`. Tests: rotate all pages 90° → each page dict has `/Rotate = 90`; rotate specific pages → only those pages have updated `/Rotate`; rotate by -90 → same as 270; invalid degrees (45) → `EzPdfError`; round-trip (rotate 90 then rotate -90) → `/Rotate` value equals original. Run — must **FAIL**.

- [ ] **6.2 [GREEN]** Create `ezpdf-core/src/rotate.rs`. Implement by getting each target page's `ObjectId` via `doc.get_pages()`, then using `doc.get_object_mut(page_id)` to access the page `Dictionary` and call `dict.set("Rotate", Object::Integer(new_degrees))`. Read the existing `/Rotate` value first (default 0 if absent) and add the requested degrees mod 360. **Note: lopdf has no built-in `rotate_pages` API — you must edit the page dictionary directly.** All tests must **PASS**.

- [ ] **6.3 [RED]** Write failing CLI tests for `ezpdf rotate input.pdf 90 -o out.pdf` and `ezpdf rotate input.pdf 90 --pages 1,3 -o out.pdf`. Run — must **FAIL**.

- [ ] **6.4 [GREEN]** Create `ezpdf-cli/src/commands/rotate.rs`. All tests must **PASS**.

- [ ] **6.5 [REFACTOR]** Clean up rotation logic. Normalize degrees to 0/90/180/270 before storing.

- [ ] **6.6 [REVIEW]** Run `cargo test --workspace`. Manual demo in Preview. Check DoD. Commit. Update `progress.md`.

---

## Phase 7: Reorder Command

### Definition of Done

- [ ] `cargo test -p ezpdf-core reorder` passes
- [ ] Binary round-trip test: reorder then reorder back → identical to original
- [ ] Manual demo: reorder pages, open in Preview, verify order

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [ ] **7.1 [RED]** Write failing tests for `ezpdf_core::reorder(input: &Path, order: &str, output: &Path) -> Result<(), EzPdfError>`. Tests: reorder `"3,1,2"` on 3-page PDF → page order changes correctly; round-trip `"2,1"` then `"2,1"` → same as original; missing page → `EzPdfError`; duplicate page → `EzPdfError`; order longer than page count → `EzPdfError`. Run — must **FAIL**.

- [ ] **7.2 [GREEN]** Create `ezpdf-core/src/reorder.rs`. Parse the order string, get the root `/Pages` dictionary via `doc.get_pages_mut()` or traverse `doc.catalog()`, reorder the `/Kids` array to reflect the new order, and update the `/Count` integer to match. **Gotcha: if `/Count` is wrong or any page's `/Parent` ref is stale after reordering, lopdf will produce a structurally invalid PDF — verify the page tree is consistent before saving.** All tests must **PASS**.

- [ ] **7.3 [RED]** Write failing CLI tests. Run — must **FAIL**.

- [ ] **7.4 [GREEN]** Create `ezpdf-cli/src/commands/reorder.rs`. All tests must **PASS**.

- [ ] **7.5 [REFACTOR]** Ensure `reorder` validation error messages explicitly name the missing/duplicate page.

- [ ] **7.6 [REVIEW]** Run `cargo test --workspace`. Manual round-trip demo. Check DoD. Commit. Update `progress.md`.

> **🏁 After 7.6: all 5 core operations complete. ezpdf works end-to-end.**

---

## Phase 8: CLI Polish

### Definition of Done

- [ ] `ezpdf --help` and `ezpdf <cmd> --help` show complete, example-rich help
- [ ] `ezpdf completions zsh` produces valid zsh completion script
- [ ] `ezpdf completions bash` and `ezpdf completions fish` work
- [ ] Progress bar appears for operations on PDFs > 20 pages
- [ ] Encrypted PDF detection works on all 5 commands
- [ ] All error messages are user-friendly with recovery hints

> [!tip] Skills for this phase
> - **All [RED] and [GREEN] tasks** → invoke `superpowers:test-driven-development` skill

### Tasks

- [ ] **8.1 [RED]** Write failing tests for: `ezpdf completions zsh` exits 0 and stdout is non-empty; `ezpdf completions bash` exits 0; `ezpdf --version` exits 0 and output contains "0.1.0"; `ezpdf nonexistent_command` exits 1 and stderr contains "error:". Run — must **FAIL**.

- [ ] **8.2 [GREEN]** Add `completions` subcommand to CLI using `clap_complete`. Add `--version` flag. Add man page generation command `ezpdf man` via `clap_mangen`. Wire encrypted-PDF detection into all 5 command handlers (call a shared `check_not_encrypted(path)` helper that returns `EzPdfError::EncryptedPdf` with a `qpdf --decrypt` tip). All tests must **PASS**.

- [ ] **8.3 [SETUP]** Add progress bar support using `indicatif` to merge, split-each, and remove commands. Threshold: show bar when PDF has > 20 pages. Bar format: `[████░░░░] Processing page 12/50 — merge`. Suppress with `--quiet`. (No unit tests for progress bar rendering — visual verification only.)

- [ ] **8.4 [RED]** Write tests for all 5 commands that: (a) pass an encrypted PDF fixture → exit 1, stderr contains "password-protected" and "qpdf"; (b) pass a non-existent file → exit 1, stderr contains the file path. Run — must **FAIL**. (Need a minimal encrypted PDF fixture — create using `qpdf --encrypt "" "" 128 -- plain.pdf encrypted.pdf` in test setup, or embed a known-encrypted PDF byte sequence as a fixture.)

- [ ] **8.5 [GREEN]** Implement the `check_not_encrypted` helper in `ezpdf-core`. Wire into all command handlers. Update all help strings to include `Examples:` sections. All tests must **PASS**.

- [ ] **8.6 [REFACTOR]** Audit all `clap` `about`, `long_about`, `help` strings — every flag must have a description. Every subcommand must have 2+ examples in `long_about`. Run clippy clean.

- [ ] **8.7 [REVIEW]** Run `cargo test --workspace`. Run all help commands manually. Pipe completions to a temp file and verify format. Check DoD. Commit. Update `progress.md`.

---

## Phase 9: Performance & Benchmarks

### Definition of Done

- [ ] `cargo bench` runs without errors
- [ ] Benchmark baseline committed to `docs/benchmarks/baseline.md`
- [ ] Parallel merge shows measurable speedup for ≥3 inputs vs sequential

> [!tip] Skills for this phase
> - No TDD skill needed — benchmarks are additive, not feature development
> - Do not change any existing logic, only add benchmarks and parallelism

### Tasks

- [ ] **9.1 [SETUP]** Create `ezpdf-core/benches/operations.rs` using `criterion`. Write benchmarks for: `merge` (5 × 10-page PDFs), `split_each` (50-page PDF), `remove` (remove half of 50-page PDF), `rotate` (all pages of 50-page PDF). Add large fixtures generation helper. Run `cargo bench -- --save-baseline baseline`. Commit benchmark results.

- [ ] **9.2 [SETUP]** Add `rayon` to `ezpdf-core` dependencies. Implement parallel file loading in `merge.rs`: load and parse all input `Document`s in parallel using `rayon::iter`, then combine sequentially. Run `cargo bench -- --baseline baseline` to verify merge is not slower (should be faster for ≥3 files).

- [ ] **9.3 [REVIEW]** Copy benchmark output to `docs/benchmarks/baseline.md`. Run `cargo test --workspace` — no regressions. Commit. Update `progress.md`.

---

## Phase 10: Distribution & Release

### Definition of Done

- [ ] `brew install ez/tap/ezpdf` works on macOS Apple Silicon
- [ ] GitHub Release v0.1.0 has 4 binary artifacts
- [ ] `cargo install ezpdf` installs the binary

> [!tip] Skills for this phase
> - No TDD skill needed — infrastructure/release tasks

### Tasks

- [ ] **10.1 [SETUP]** Create `.github/workflows/release.yml`. Matrix: `aarch64-apple-darwin` (macos-14), `x86_64-apple-darwin` (macos-13), `x86_64-unknown-linux-gnu` (ubuntu-22.04), `aarch64-unknown-linux-gnu` (ubuntu-22.04 + `cross`). Install cross via `taiki-e/install-action@cross` (not `cargo install cross`). Steps: checkout, `dtolnay/rust-toolchain@stable`, install cross if needed, `cargo build --release --target <target>`, strip binary, upload artifact. Final job: create GitHub release with `softprops/action-gh-release@v2` and attach all artifacts. **Note: use ubuntu-22.04 specifically — Tauri v2 desktop app (v2) requires `libwebkit2gtk-4.1` which is not available on ubuntu-20.04.**

- [ ] **10.2 [SETUP]** Push a `v0.1.0` tag to trigger the release workflow: `git tag v0.1.0 && git push origin v0.1.0`. Monitor the Actions workflow. Once release exists, note the download URLs for each binary.

- [ ] **10.3 [SETUP]** Create Homebrew tap formula. If `github.com/ez/homebrew-tap` doesn't exist yet, create the repo via `gh repo create ez/homebrew-tap --public`. Write `Formula/ezpdf.rb` with `url`, `sha256`, `version` for each platform. Install man page via `man1.install`. Install shell completions. Test locally: `brew install --formula ./Formula/ezpdf.rb`.

- [ ] **10.4 [SETUP]** Publish to crates.io: ensure `ezpdf-core/Cargo.toml` and `ezpdf-cli/Cargo.toml` have `description`, `repository`, `keywords`, `categories`. Run `cargo publish -p ezpdf-core`, then `cargo publish -p ezpdf-cli`.

- [ ] **10.5 [SETUP]** Update `README.md` with: installation (Homebrew + cargo install), full usage examples for all 5 commands, link to man page, link to GitHub releases.

- [ ] **10.6 [REVIEW]** End-to-end install test: `brew install ez/tap/ezpdf`, run all 5 commands with real PDFs, verify correct output. Run `cargo install ezpdf` in a clean environment. Update `progress.md`. **v1.0.0 is RELEASED.**

---

## Blockers

_None yet. Blockers found during stories will be injected here._

---

## Backlog (non-v1)

| ID | Item | Priority | Target |
|----|------|----------|--------|
| B1 | Batch operations (`--batch` flag for directory processing) | HIGH | v1.1 |
| B2 | Encrypted PDF support (`--password` flag) | HIGH | v2 |
| B3 | `ezpdf-app` desktop app (Tauri v2 + Svelte 5) | HIGH | v2 |
| B4 | `ezpdf info`: show page count, dimensions, metadata | MEDIUM | v1.1 |
| B5 | PDF metadata read/write (title, author, keywords) | MEDIUM | v1.2 |
| B6 | Windows support | MEDIUM | v1.2 |
| B7 | Extract images from PDF | LOW | v2 |
| B8 | PDF linearization / web optimization | LOW | v2 |
| B9 | Watermark pages (text or image) | LOW | v2 |
| B10 | Bookmarks / outline manipulation | LOW | v2 |
