---
title: "feat: Build ezpdf — Rust PDF CLI Tool (v1)"
type: feat
status: active
date: 2026-03-12
origin: docs/brainstorms/2026-03-12-pdf-cli-tool-brainstorm.md
---

# feat: Build ezpdf — Rust PDF CLI Tool (v1)

## Overview

`ezpdf` is a fast, lossless PDF manipulation CLI tool written in Rust. It targets macOS and Linux
developers as a modern replacement for `pdftk`. v1 ships 5 core operations (merge, split, remove,
rotate, reorder) as a single binary distributed via Homebrew.

**Architecture:** Cargo workspace → `ezpdf-core` (shared library) + `ezpdf-cli` (CLI binary).
The desktop app (`ezpdf-app`, Tauri + Svelte) is v2 and tracked in the backlog.

**Guiding principles for this plan:**
- Every story is independently testable and demonstrable
- The plan is revisited and refined after each story completes
- Blockers found during a story are injected as blocking stories before we continue
- Non-blockers are appended to the Backlog with an assigned priority

---

## Problem Statement

PDF manipulation on the command line today requires either `pdftk` (Java, hard to install, old) or
Python scripts (slow, not a standalone binary). Developers need a fast, ergonomic, dependency-free
tool that runs on macOS and Linux without compromising PDF quality.

---

## SDLC Story Map

```
Story 1: Project Foundation      →  Story 2: Page Range Parser
Story 2: Page Range Parser       →  Story 3: Merge Command
Story 3: Merge Command           →  Story 4: Remove Command
Story 4: Remove Command          →  Story 5: Split Command
Story 5: Split Command           →  Story 6: Rotate Command
Story 6: Rotate Command          →  Story 7: Reorder Command
Story 7: Reorder Command         →  Story 8: CLI Polish
Story 8: CLI Polish              →  Story 9: Performance & Benchmarks
Story 9: Performance             →  Story 10: Distribution & Release
```

Each story ends with a **Story Review Checkpoint** — see the template at the bottom of this plan.

---

## Story 1: Project Foundation

**Goal:** A working Cargo workspace that compiles, has CI on macOS + Linux, and is ready for
feature development.

### Acceptance Criteria

- [ ] `git init` + Cargo workspace at `~/Projects/ez-pdf/`
- [ ] Three crates in workspace: `ezpdf-core` (lib), `ezpdf-cli` (bin), `ezpdf-app` (empty placeholder)
- [ ] `cargo build --workspace` succeeds with zero warnings
- [ ] `cargo test --workspace` succeeds (zero tests, zero failures)
- [ ] GitHub Actions CI: runs `cargo test` on `ubuntu-latest` and `macos-latest` on every push to `main`
- [ ] `.gitignore`, `README.md` skeleton, `CHANGELOG.md` with `[Unreleased]` section
- [ ] `rustfmt.toml` and `clippy.toml` configured
- [ ] `LICENSE` (MIT)

### Demo

```bash
cd ~/Projects/ez-pdf
cargo build --workspace
# → Compiling ezpdf-core ... Compiling ezpdf-cli ... Finished

cargo test --workspace
# → running 0 tests ... test result: ok

cargo clippy --workspace -- -D warnings
# → no warnings
```

### Key Files

```
ez-pdf/
├── Cargo.toml              # workspace root
├── ezpdf-core/
│   ├── Cargo.toml
│   └── src/lib.rs
├── ezpdf-cli/
│   ├── Cargo.toml
│   └── src/main.rs         # placeholder: prints "ezpdf v0.1.0"
├── ezpdf-app/
│   └── Cargo.toml          # placeholder, no src yet
├── .github/
│   └── workflows/
│       └── ci.yml
├── rustfmt.toml
├── clippy.toml
├── .gitignore
├── CHANGELOG.md
├── LICENSE
└── README.md
```

### Workspace Cargo.toml

```toml
[workspace]
members = ["ezpdf-core", "ezpdf-cli"]
resolver = "2"

[workspace.package]
version = "0.1.0"
edition = "2021"
license = "MIT"
authors = ["EZ"]
repository = "https://github.com/ez/ezpdf"

[workspace.dependencies]
lopdf        = "0.31"
thiserror    = "2"
anyhow       = "1"
clap         = { version = "4", features = ["derive", "env"] }
clap_complete = "4"
clap_mangen  = "0.2"
indicatif    = "0.17"
rayon        = "1"
criterion    = { version = "0.5", features = ["html_reports"] }
```

### GitHub Actions CI (`.github/workflows/ci.yml`)

```yaml
name: CI
on: [push, pull_request]
jobs:
  test:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2
      - run: cargo test --workspace
      - run: cargo clippy --workspace -- -D warnings
      - run: cargo fmt --check
```

### Story Review Checkpoint

After completing Story 1:
- [ ] All acceptance criteria checked off
- [ ] Demo commands run and output matches expectation
- [ ] Update this plan: mark Story 1 `[DONE]`, note any blockers or backlog items found
- [ ] If any blockers found: insert new story after Story 1 before continuing

---

## Story 2: Page Range Parser

**Goal:** A reliable, well-tested page range parser in `ezpdf-core` that all commands use.
This is pure logic — no PDF handling yet. Gets validated before any lopdf integration.

### Acceptance Criteria

- [ ] `ezpdf_core::page_range::parse(input: &str, page_count: u32) -> Result<Vec<u32>, Error>`
- [ ] Supports syntax: `1`, `3-7`, `1,3,5`, `1-5,7,9-12`, `3-` (to end), `1` (single page)
- [ ] 1-based page indexing (page 1 = first page)
- [ ] Returns deduplicated, sorted page numbers
- [ ] Returns `Error::InvalidPageRange` with helpful message for:
  - Out of range (`page 15 does not exist — document has 10 pages`)
  - Invalid syntax (`'abc' is not a valid page range`)
  - Range where start > end (`'7-3' is invalid — start must be ≤ end`)
  - Empty result
- [ ] 100% unit test coverage for the parser module
- [ ] Fuzz test for the parser (via `cargo-fuzz` or property tests with `proptest`)

### Demo

```bash
cargo test -p ezpdf-core page_range
# → running 18 tests ... test result: ok. 18 passed
```

### Key Files

```
ezpdf-core/src/
├── lib.rs                  # pub mod page_range; pub mod error;
├── page_range.rs           # parse() + PageRange iterator
└── error.rs                # EzPdfError enum with thiserror
```

### Core Types

```rust
// ezpdf-core/src/error.rs
#[derive(Debug, thiserror::Error)]
pub enum EzPdfError {
    #[error("page {page} does not exist — document has {total} pages")]
    PageOutOfRange { page: u32, total: u32 },

    #[error("invalid page range syntax: '{input}' — {hint}")]
    InvalidSyntax { input: String, hint: String },

    #[error("encrypted PDFs are not supported in v1 — decrypt your file first")]
    EncryptedPdf,

    #[error("failed to read PDF: {0}")]
    Io(#[from] std::io::Error),

    #[error("PDF processing error: {0}")]
    Pdf(String),
}
```

```rust
// ezpdf-core/src/page_range.rs
/// Parse "1-5,7,9-12" into a sorted, deduplicated Vec<u32> of 1-based page numbers.
pub fn parse(input: &str, page_count: u32) -> Result<Vec<u32>, EzPdfError> { ... }
```

### Story Review Checkpoint

After completing Story 2:
- [ ] All acceptance criteria checked off
- [ ] All tests pass: `cargo test -p ezpdf-core`
- [ ] Update plan: mark Story 2 `[DONE]`, note blockers/backlog items
- [ ] If any blockers found: insert new story before Story 3

---

## Story 3: Merge Command

**Goal:** Working `ezpdf merge` command — the first end-to-end user-facing feature.

### Acceptance Criteria

- [ ] `ezpdf_core::merge(inputs: &[PathBuf], output: &PathBuf) -> Result<(), EzPdfError>`
- [ ] Lossless merge: PDF content streams are copied, never re-encoded
- [ ] Detects encrypted PDFs and returns `EzPdfError::EncryptedPdf` with clear message
- [ ] Validates all input files exist before starting (fail fast)
- [ ] CLI: `ezpdf merge file1.pdf file2.pdf [file3.pdf...] -o output.pdf`
- [ ] CLI: `--verbose` flag shows page count of each input file
- [ ] Integration test: merge 2 PDFs → output page count equals sum of inputs
- [ ] Integration test: merge preserves page dimensions (compare first page /MediaBox)
- [ ] PDF fixtures in `ezpdf-core/tests/fixtures/` (generated with `lopdf` in a test helper)

### Demo

```bash
# Create test fixtures
cargo run -p ezpdf-cli -- merge tests/fixtures/3page.pdf tests/fixtures/5page.pdf -o /tmp/merged.pdf
# → Merged 2 files (8 pages) → /tmp/merged.pdf

cargo run -p ezpdf-cli -- merge tests/fixtures/3page.pdf tests/fixtures/5page.pdf -o /tmp/merged.pdf --verbose
# → [1/2] 3page.pdf (3 pages)
# → [2/2] 5page.pdf (5 pages)
# → Merged 2 files (8 pages) → /tmp/merged.pdf

# Open and verify page count
open /tmp/merged.pdf
```

### Key Files

```
ezpdf-core/src/
├── merge.rs                # merge() implementation with lopdf
ezpdf-core/tests/
├── fixtures/
│   ├── README.md           # documents what each fixture is
│   ├── 3page.pdf           # 3-page test PDF (generated in test helper)
│   └── 5page.pdf           # 5-page test PDF
├── common/
│   └── mod.rs              # create_test_pdf() helper using lopdf
└── merge_test.rs           # integration tests
ezpdf-cli/src/
└── commands/
    └── merge.rs            # clap subcommand, calls ezpdf_core::merge()
```

### lopdf Merge Pattern (Lossless)

```rust
// ezpdf-core/src/merge.rs
use lopdf::{Document, Object, ObjectId};

pub fn merge(inputs: &[&Path], output: &Path) -> Result<(), EzPdfError> {
    // 1. Load all input documents
    // 2. Create new Document
    // 3. For each input doc: renumber object IDs to avoid conflicts,
    //    copy all objects into target doc
    // 4. Build new /Pages tree from collected page references
    // 5. Write to output path
    // Key: content streams are copied as-is, never decoded
}
```

### Story Review Checkpoint

After completing Story 3:
- [ ] All acceptance criteria checked off
- [ ] `cargo test -p ezpdf-core merge` passes
- [ ] Manual demo run produces correct output PDF
- [ ] Update plan: mark Story 3 `[DONE]`, note blockers/backlog items

---

## Story 4: Remove Command

**Goal:** Working `ezpdf remove` command that deletes specified pages.

### Acceptance Criteria

- [ ] `ezpdf_core::remove(input: &Path, pages: &str, output: &Path) -> Result<(), EzPdfError>`
- [ ] Uses `page_range::parse()` for the page specification
- [ ] Fails with helpful error if all pages would be removed
- [ ] Lossless: remaining page content streams are not touched
- [ ] CLI: `ezpdf remove input.pdf 3,5,7-9 -o output.pdf`
- [ ] CLI: `--pages` alias for positional page argument (both `ezpdf remove input.pdf 3-5` and `ezpdf remove input.pdf --pages 3-5` work)
- [ ] Integration test: remove pages from 10-page PDF, verify output has correct page count
- [ ] Integration test: removing all pages returns an error

### Demo

```bash
cargo run -p ezpdf-cli -- remove tests/fixtures/10page.pdf 3,5,7-9 -o /tmp/removed.pdf
# → Removed 5 pages → /tmp/removed.pdf (5 pages remaining)

# Error case
cargo run -p ezpdf-cli -- remove tests/fixtures/3page.pdf 1-3 -o /tmp/empty.pdf
# → Error: cannot remove all pages from a document
```

### Key Files

```
ezpdf-core/src/remove.rs
ezpdf-cli/src/commands/remove.rs
ezpdf-core/tests/remove_test.rs
```

### Story Review Checkpoint

After completing Story 4:
- [ ] All acceptance criteria checked off
- [ ] Tests pass
- [ ] Manual demo verified
- [ ] Update plan: mark Story 4 `[DONE]`, note blockers/backlog items

---

## Story 5: Split Command

**Goal:** Two split modes — extract a page range, or burst into individual pages.

### Acceptance Criteria

- [ ] `ezpdf_core::split_range(input: &Path, pages: &str, output: &Path) -> Result<(), EzPdfError>`
- [ ] `ezpdf_core::split_each(input: &Path, output_dir: &Path) -> Result<u32, EzPdfError>`
  - Returns the number of pages written
  - Output filenames: `page-001.pdf`, `page-002.pdf`, ..., zero-padded to match total page count
- [ ] CLI mode 1: `ezpdf split input.pdf 1-10 -o part.pdf`
- [ ] CLI mode 2: `ezpdf split input.pdf --each -o ./pages/` (creates output dir if needed)
- [ ] `--each` without `-o` uses `./pages/` as default output directory
- [ ] Integration test: split range → output page count matches range size
- [ ] Integration test: split each → N files created for N-page PDF
- [ ] Integration test: output directory is created if it doesn't exist

### Demo

```bash
# Range split
cargo run -p ezpdf-cli -- split tests/fixtures/10page.pdf 3-7 -o /tmp/pages3to7.pdf
# → Extracted pages 3-7 (5 pages) → /tmp/pages3to7.pdf

# Burst
mkdir -p /tmp/pages
cargo run -p ezpdf-cli -- split tests/fixtures/10page.pdf --each -o /tmp/pages/
# → Split into 10 pages → /tmp/pages/
# → page-01.pdf  page-02.pdf  ...  page-10.pdf
ls /tmp/pages/
```

### Key Files

```
ezpdf-core/src/split.rs
ezpdf-cli/src/commands/split.rs
ezpdf-core/tests/split_test.rs
```

### Story Review Checkpoint

After completing Story 5:
- [ ] All acceptance criteria checked off
- [ ] Tests pass
- [ ] Manual demo verified
- [ ] Update plan: mark Story 5 `[DONE]`, note blockers/backlog items

---

## Story 6: Rotate Command

**Goal:** Rotate all or specified pages by 90/180/270 degrees.

### Acceptance Criteria

- [ ] `ezpdf_core::rotate(input: &Path, degrees: i32, pages: Option<&str>, output: &Path) -> Result<(), EzPdfError>`
- [ ] Supported rotations: `90`, `180`, `270`, `-90` (same as 270)
- [ ] `pages` is `None` → rotate all pages; `Some("1,3,5")` → rotate specific pages
- [ ] Lossless: sets the `/Rotate` key in each page dictionary, does not re-render
- [ ] CLI: `ezpdf rotate input.pdf 90 -o rotated.pdf` (all pages)
- [ ] CLI: `ezpdf rotate input.pdf 90 --pages 1,3,5 -o rotated.pdf` (specific pages)
- [ ] CLI: `ezpdf rotate input.pdf -90 -o rotated.pdf` (270° rotation)
- [ ] Rejects invalid rotation values with clear error
- [ ] Integration test: rotate 90° then 270° → same as original (rotation stacks correctly)

### Demo

```bash
cargo run -p ezpdf-cli -- rotate tests/fixtures/3page.pdf 90 -o /tmp/rotated.pdf
# → Rotated 3 pages (90°) → /tmp/rotated.pdf

cargo run -p ezpdf-cli -- rotate tests/fixtures/3page.pdf 90 --pages 1,3 -o /tmp/rotated_partial.pdf
# → Rotated 2 pages (90°) → /tmp/rotated_partial.pdf
open /tmp/rotated.pdf
```

### Key Files

```
ezpdf-core/src/rotate.rs
ezpdf-cli/src/commands/rotate.rs
ezpdf-core/tests/rotate_test.rs
```

### Story Review Checkpoint

After completing Story 6:
- [ ] All acceptance criteria checked off
- [ ] Tests pass, including the rotate-then-unrotate round-trip test
- [ ] Manual demo verified (visually confirm rotation in Preview)
- [ ] Update plan: mark Story 6 `[DONE]`, note blockers/backlog items

---

## Story 7: Reorder Command

**Goal:** Rearrange pages in a PDF by specifying a new page order.

### Acceptance Criteria

- [ ] `ezpdf_core::reorder(input: &Path, order: &str, output: &Path) -> Result<(), EzPdfError>`
  - `order` is a comma-separated list of 1-based page numbers in the desired order: `"3,1,2,4"`
- [ ] Validates that every page in the document is included exactly once
  - Error: `page 2 is missing from the order specification`
  - Error: `page 3 appears 2 times in the order specification`
- [ ] CLI: `ezpdf reorder input.pdf 3,1,2,4 -o reordered.pdf`
- [ ] Lossless: page objects are rearranged, content not touched
- [ ] Integration test: reorder then reorder back to original → identical to original (binary compare)
- [ ] Integration test: missing page returns helpful error

### Demo

```bash
cargo run -p ezpdf-cli -- reorder tests/fixtures/3page.pdf 3,1,2 -o /tmp/reordered.pdf
# → Reordered 3 pages → /tmp/reordered.pdf

# Error case
cargo run -p ezpdf-cli -- reorder tests/fixtures/3page.pdf 3,1 -o /tmp/bad.pdf
# → Error: page 2 is missing from the order specification (document has 3 pages)
```

### Key Files

```
ezpdf-core/src/reorder.rs
ezpdf-cli/src/commands/reorder.rs
ezpdf-core/tests/reorder_test.rs
```

### Story Review Checkpoint

After completing Story 7:
- [ ] All acceptance criteria checked off
- [ ] Binary round-trip test passes
- [ ] Manual demo verified
- [ ] Update plan: mark Story 7 `[DONE]`, note blockers/backlog items
- **v1 core complete** — all 5 operations working and tested

---

## Story 8: CLI Polish

**Goal:** Production-quality user experience — shell completions, man pages, progress output,
and a first-class help system. After this story, `ezpdf` is ready for real use.

### Acceptance Criteria

**Help system:**
- [ ] `ezpdf --help` shows all subcommands with one-line descriptions
- [ ] `ezpdf <cmd> --help` shows full usage, all flags, examples
- [ ] Long `--help` includes example commands for each flag
- [ ] Version flag: `ezpdf --version` prints `ezpdf 0.1.0`

**Shell completions:**
- [ ] `ezpdf completions bash` prints bash completion script
- [ ] `ezpdf completions zsh` prints zsh completion script
- [ ] `ezpdf completions fish` prints fish completion script
- [ ] Completions complete PDF filenames for file arguments

**Man pages:**
- [ ] `ezpdf man` generates man page content (printed to stdout for piping)
- [ ] Man page generated via `clap_mangen` matches the help text
- [ ] Homebrew formula (Story 10) will install the man page to `$(brew --prefix)/share/man/man1/`

**Progress output:**
- [ ] Large file operations (>5MB or >50 pages) show a progress bar via `indicatif`
- [ ] `--quiet` flag suppresses all non-error output
- [ ] Progress bar shows: current operation, file name, pages processed/total

**Encrypted PDF handling:**
- [ ] All commands detect encrypted PDFs and exit with:
  ```
  Error: 'input.pdf' is password-protected. ezpdf v1 does not support encrypted PDFs.
  Tip: Try: qpdf --decrypt input.pdf decrypted.pdf
  ```

**Error messages (UX):**
- [ ] Out-of-range page shows document page count in error message
- [ ] File-not-found shows the full path attempted
- [ ] All errors exit with code 1, success exits with code 0

### Demo

```bash
ezpdf --help
# → Usage: ezpdf <COMMAND>
# → Commands: merge, split, remove, rotate, reorder, completions, man
# → Options: -q/--quiet, -v/--verbose, --version, --help

ezpdf merge --help
# → Merge two or more PDF files into one
# → Usage: ezpdf merge [OPTIONS] <FILES>... -o <OUTPUT>
# → Examples:
# →   ezpdf merge a.pdf b.pdf -o combined.pdf
# →   ezpdf merge *.pdf -o all.pdf

ezpdf completions zsh >> ~/.zshrc
# shell now completes ezpdf commands

ezpdf merge a_large.pdf b_large.pdf -o /tmp/out.pdf
# → [████████████████░░░░] Merging... page 34/50
```

### Key Files

```
ezpdf-cli/src/
├── main.rs                 # Cli struct with global flags
├── commands/
│   ├── mod.rs
│   ├── merge.rs
│   ├── split.rs
│   ├── remove.rs
│   ├── rotate.rs
│   ├── reorder.rs
│   └── completions.rs      # generates shell completions
└── progress.rs             # indicatif progress bar helpers
```

### Story Review Checkpoint

After completing Story 8:
- [ ] All acceptance criteria checked off
- [ ] Manual demo: test all help, completions, man page output
- [ ] Update plan: mark Story 8 `[DONE]`, note blockers/backlog items
- **After this story: ezpdf is fully usable v1 for daily use**

---

## Story 9: Performance & Benchmarks

**Goal:** Establish performance baselines and implement parallel merge. Ensure large PDFs are
handled efficiently.

### Acceptance Criteria

- [ ] `criterion` benchmark suite in `ezpdf-core/benches/`
- [ ] Benchmarks cover: merge (10 files × 10 pages), split-each (100-page PDF), remove (50%), rotate (all pages)
- [ ] Baseline recorded and committed to `docs/benchmarks/baseline.md`
- [ ] Parallel file loading for merge: use `rayon` to load+parse multiple input files concurrently
  - Before rayon: measure with 10 × large PDFs
  - After rayon: merge should be faster for multiple inputs
- [ ] Memory: operations on a 100MB PDF should not load more than 2× the file size into memory
  - Verified via `cargo test` with memory tracking or manual `Activity Monitor` observation
- [ ] CI: run benchmarks and publish results to PR comments (via `criterion-compare-action`)

### Demo

```bash
# Generate large fixtures
cargo run --example generate_fixtures -- --pages 100 --size large

cargo bench
# → merge/10_files     time: [45.23 ms 46.10 ms 47.00 ms]
# → split_each/100p    time: [12.5 ms  12.8 ms  13.1 ms]
# → rotate_all/100p    time: [8.2 ms   8.5 ms   8.8 ms]
```

### Key Files

```
ezpdf-core/
├── benches/
│   └── operations.rs       # criterion benchmarks
├── examples/
│   └── generate_fixtures.rs  # generate large test PDFs
docs/
└── benchmarks/
    └── baseline.md         # committed baseline numbers
```

### Story Review Checkpoint

After completing Story 9:
- [ ] Benchmarks run without error
- [ ] Baseline committed
- [ ] Parallel merge shows measurable improvement for ≥3 input files
- [ ] Update plan: mark Story 9 `[DONE]`

---

## Story 10: Distribution & Release

**Goal:** v1.0.0 release — binary on GitHub Releases, Homebrew tap, crates.io.
Users can install with a single command on macOS and Linux.

### Acceptance Criteria

**GitHub Actions release workflow:**
- [ ] Triggers on `git tag v*` push
- [ ] Builds release binaries for 4 targets:
  - `aarch64-apple-darwin` (macOS Apple Silicon)
  - `x86_64-apple-darwin` (macOS Intel)
  - `x86_64-unknown-linux-gnu` (Linux x86_64)
  - `aarch64-unknown-linux-gnu` (Linux ARM64)
- [ ] Binaries are stripped and compressed (`upx` optional)
- [ ] GitHub Release created with changelog notes and binary attachments

**Homebrew tap:**
- [ ] Repo `github.com/ez/homebrew-tap` created
- [ ] Formula `Formula/ezpdf.rb` downloads the correct binary for the user's platform
- [ ] `brew install ez/tap/ezpdf` works on macOS (Apple Silicon + Intel)
- [ ] `brew install ez/tap/ezpdf` works on Linux via Linuxbrew
- [ ] Man page installed to `$(brew --prefix)/share/man/man1/ezpdf.1`
- [ ] Shell completions installed in Homebrew prefix

**crates.io:**
- [ ] `ezpdf-core` published to crates.io
- [ ] `ezpdf-cli` published to crates.io as `ezpdf`
- [ ] `cargo install ezpdf` installs the binary

**README:**
- [ ] Installation instructions for all 3 methods
- [ ] Usage examples for all 5 commands
- [ ] macOS GIF/screenshot demo

### Demo

```bash
# Install from Homebrew
brew install ez/tap/ezpdf
ezpdf --version
# → ezpdf 1.0.0

# Install from crates.io
cargo install ezpdf
ezpdf --version
# → ezpdf 1.0.0

# Test all operations work from installed binary
ezpdf merge a.pdf b.pdf -o combined.pdf
ezpdf split combined.pdf --each -o ./pages/
ezpdf remove combined.pdf 1 -o shortened.pdf
ezpdf rotate combined.pdf 90 -o rotated.pdf
ezpdf reorder combined.pdf 2,1 -o swapped.pdf
```

### Key Files

```
.github/workflows/
├── ci.yml                  # existing CI
└── release.yml             # new: build + publish on tag
homebrew-tap/ (separate repo)
└── Formula/
    └── ezpdf.rb
ez-pdf/
└── README.md               # updated with full installation + usage
```

### GitHub Actions Release Workflow Skeleton

```yaml
name: Release
on:
  push:
    tags: ["v*"]
jobs:
  build:
    strategy:
      matrix:
        include:
          - target: aarch64-apple-darwin
            os: macos-14
          - target: x86_64-apple-darwin
            os: macos-13
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: aarch64-unknown-linux-gnu
            os: ubuntu-latest
            use_cross: true
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - name: Install cross (if needed)
        if: matrix.use_cross
        run: cargo install cross --git https://github.com/cross-rs/cross
      - name: Build
        run: |
          if [ "${{ matrix.use_cross }}" = "true" ]; then
            cross build --release --target ${{ matrix.target }}
          else
            cargo build --release --target ${{ matrix.target }}
          fi
      - name: Upload binary
        uses: actions/upload-artifact@v4
        with:
          name: ezpdf-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/ezpdf
  release:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/download-artifact@v4
      - uses: softprops/action-gh-release@v2
        with:
          files: "**/ezpdf*"
          generate_release_notes: true
```

### Story Review Checkpoint

After completing Story 10:
- [ ] All acceptance criteria checked off
- [ ] `brew install ez/tap/ezpdf` tested on macOS
- [ ] `cargo install ezpdf` tested
- [ ] Update plan: mark Story 10 `[DONE]`
- **v1.0.0 is RELEASED** 🎉

---

## System-Wide Impact

### Quality Preservation Strategy

The lossless guarantee is the core value proposition. It must be enforced at the architecture level:

- `ezpdf-core` **never** calls any rendering, rasterization, or content decompression API
- The only operations on content streams are copy-by-reference (ObjectId remapping in lopdf)
- Integration tests compare output file sizes — a file that has been re-encoded will be dramatically different in size

### Error & Failure Propagation

```
CLI layer (clap)  →  EzPdfError  →  stderr + exit code 1
                                →  no panic allowed (all lopdf errors wrapped)

lopdf errors      →  EzPdfError::Pdf(String)
                →  printed as: "PDF processing error: <lopdf message>"

IO errors         →  EzPdfError::Io  →  "failed to read/write file: <path>"
```

All errors are `#[derive(thiserror::Error)]` in `ezpdf-core::error`. The CLI converts them to
user-friendly messages via a top-level `match` in `main.rs`.

### API Surface Parity

When `ezpdf-app` (v2 desktop) is built, it calls `ezpdf-core` directly. This plan ensures
`ezpdf-core` exports a clean, stable public API (`pub fn merge(...)`, `pub fn split(...)` etc.)
so the GUI never reimplements PDF logic.

---

## Blockers & Resolution Process

When a blocker is found during a story:

1. **Stop the current story** — do not continue past a blocker
2. **Insert a new story** in the plan, numbered between the current story and the next (e.g., Story 3b)
3. **Describe the blocker** with exact reproduction steps
4. **Resolve the blocker story** before resuming the original story
5. **Update the plan** to reflect the resolution

Example blocker entry:
```
## Story 3b: [BLOCKER] lopdf Page Reference Corruption on Merge

**Found during:** Story 3 (Merge Command)
**Symptom:** Merging two PDFs with form fields causes ObjectId collision in output.
**Root cause:** TBD
**Resolution:** TBD
**Status:** OPEN
```

---

## Backlog (Non-Blocking)

Prioritized items to address after v1.0.0 release. Add new items here as they are discovered
during stories — do not block the current story for these.

| Priority | Item | Version Target |
|----------|------|----------------|
| HIGH | `B1` — Batch operations: apply same operation to all PDFs in a directory (`--batch`) | v1.1 |
| HIGH | `B2` — Encrypted PDF support: `--password` flag to decrypt-then-operate | v2 |
| HIGH | `B3` — `ezpdf-app` desktop app (Tauri v2 + Svelte 5) | v2 |
| MEDIUM | `B4` — PDF metadata: read/write title, author, subject, keywords | v1.2 |
| MEDIUM | `B5` — `ezpdf info`: show page count, dimensions, metadata, encryption status | v1.1 |
| MEDIUM | `B6` — Windows support (cross-compilation target) | v1.2 |
| LOW | `B7` — Extract images from PDF | v2 |
| LOW | `B8` — PDF linearization / web optimization | v2 |
| LOW | `B9` — Bookmarks / outline manipulation | v2 |
| LOW | `B10` — Watermark: add text or image watermark to pages | v2 |
| LOW | `B11` — `crates.io` package for `ezpdf-core` as a standalone Rust library | v1.1 |

---

## Technical Decisions (from Brainstorm)

See full rationale in `docs/brainstorms/2026-03-12-pdf-cli-tool-brainstorm.md`.

| Decision | Choice | Rationale |
|----------|--------|-----------|
| Language | Rust | Performance + single binary + Tauri-native |
| PDF library | `lopdf` 0.31 | Lossless, pure Rust, proven write support |
| CLI framework | `clap` v4 with derive | Industry standard for Rust CLI, excellent shell completion support |
| Workspace | Cargo workspace | Shared deps + clean separation of core/cli/app |
| Distribution | Homebrew tap | Standard for macOS/Linux developer tools |
| Page indexing | 1-based | Matches user mental model |
| In-place edits | Disallowed by default | Safety — require explicit `--in-place` |
| Encrypted PDFs | Detect and error (v1) | Reduce scope; suggest `qpdf` as workaround |
| Batch operations | Deferred to v1.1 | Reduce v1 scope |
| Desktop GUI | Tauri v2 + Svelte 5 | OS WebView (small bundle), Rust backend reuses core |

---

## Story Review Template

Use this template at the end of each story:

```markdown
## Story N Review — [DONE/BLOCKED]

**Completed:** YYYY-MM-DD
**Status:** DONE | BLOCKED

### Acceptance Criteria
- [x] Criterion 1 — ✅
- [x] Criterion 2 — ✅
- [ ] Criterion 3 — ❌ (explain why, inject as blocker if critical)

### Demo Output
[Paste actual terminal output from the demo commands]

### Blockers Found
- None | [B-N]: <blocker description> → injected as Story Nb

### Backlog Items Added
- None | [B-N]: <item> → added to Backlog table with priority

### Plan Refinements
- [Any changes to upcoming stories based on what we learned]
```

---

## Research Findings (2026-03-12)

Key gotchas discovered during research — incorporated into story acceptance criteria above.

### lopdf
- **No built-in `rotate_pages` API** — rotation requires editing `/Rotate` key directly in each page dictionary via `doc.get_object_mut(page_id)` → `dict.set("Rotate", Object::Integer(degrees))`
- **Page tree consistency** — after reorder, `/Count` in `/Pages` dict and each page's `/Parent` ref must be correct or lopdf produces a structurally invalid PDF
- **Merging** — ObjectId remapping is the critical step; pages in different documents have conflicting IDs that must be renumbered before combining

### GitHub Actions
- Use `macos-14` for Apple Silicon (M1) builds; `macos-13` is the last Intel macOS runner
- Install `cross` via `taiki-e/install-action@cross` (not `cargo install cross` which is slow)
- Use `ubuntu-22.04` specifically for Linux builds (Tauri v2 desktop app requires `libwebkit2gtk-4.1` unavailable on 20.04)

### Tauri v2 (for v2 backlog)
- Stable since October 2024; Svelte 5 also stable since October 2024
- Workspace integration works cleanly — `src-tauri/Cargo.toml` references `ezpdf-core = { path = "../../ezpdf-core" }`
- `crate-type = ["staticlib", "cdylib", "rlib"]` required in Tauri lib crate (generated by `create-tauri-app`)
- Linux requires `libwebkit2gtk-4.1-dev` at build time; AppImage does NOT bundle it (host must have it)
- macOS universal binary: `pnpm tauri build --target universal-apple-darwin`

## Sources & References

### Origin

- **Brainstorm:** `docs/brainstorms/2026-03-12-pdf-cli-tool-brainstorm.md`
  - Key decisions carried forward: Rust/lopdf for lossless ops, Cargo workspace (core/cli/app),
    Homebrew distribution, Tauri+Svelte for v2 desktop app, no encryption or batch in v1

### External References

- [lopdf crate](https://crates.io/crates/lopdf) — PDF object model manipulation
- [clap v4 docs](https://docs.rs/clap/latest/clap/) — CLI framework with derive macros
- [clap_complete](https://docs.rs/clap_complete) — shell completion generation
- [clap_mangen](https://docs.rs/clap_mangen) — man page generation
- [indicatif](https://docs.rs/indicatif) — progress bars and spinners
- [criterion](https://docs.rs/criterion) — statistical benchmarking
- [rayon](https://docs.rs/rayon) — data parallelism
- [cross](https://github.com/cross-rs/cross) — cross-compilation for Linux targets
- [Tauri v2 docs](https://v2.tauri.app) — desktop app framework
- [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain) — GitHub Actions Rust setup
- [Swatinem/rust-cache](https://github.com/Swatinem/rust-cache) — Cargo cache in CI
