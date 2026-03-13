---
date: 2026-03-12
topic: pdf-cli-tool
---

# ezpdf — PDF CLI Tool + Desktop App

## What We're Building

`ezpdf` is a fast, lossless PDF manipulation tool written in Rust, targeting macOS and Linux. It ships
as two separate products sharing a single core library:

1. **`ezpdf` CLI** — terminal tool for developers and power users
2. **`ezpdf` desktop app** — GUI built with Tauri (Rust backend + web frontend) for everyday users

Both products call the same `ezpdf-core` Rust crate, so all PDF logic is written once. The CLI is
v1; the desktop app is v2.

It handles the most common PDF operations — merge, split, remove, rotate, and reorder pages — while
preserving the exact quality of the original file (no re-rendering).

## Core Operations

| Operation | Description |
|-----------|-------------|
| `merge`   | Combine two or more PDFs into one, in order |
| `split`   | Extract a page range (`3-7`) OR burst into individual pages (`--each`) |
| `remove`  | Delete specific pages or ranges from a PDF |
| `rotate`  | Rotate pages by 90/180/270° (all or specific pages) |
| `reorder` | Rearrange pages by specifying a new order (e.g. `3,1,2,4`) |

### CLI Shape (Illustrative)

```
ezpdf merge a.pdf b.pdf c.pdf -o combined.pdf
ezpdf split report.pdf 1-10 -o part1.pdf
ezpdf split report.pdf --each -o ./pages/
ezpdf remove report.pdf 3,5,7-9 -o cleaned.pdf
ezpdf rotate report.pdf 90 -o rotated.pdf
ezpdf rotate report.pdf 90 --pages 1,3,5 -o rotated.pdf
ezpdf reorder report.pdf 3,1,2,4 -o reordered.pdf
```

Page ranges use the intuitive `1-5,7,9-12` syntax consistently across all subcommands.

## Why This Approach

### Language: Rust

Performance is the stated priority. Rust provides:
- **Zero-cost abstractions** — no GC pauses, predictable latency
- **Single binary** — easy Homebrew distribution, no runtime dependency
- **Memory safety** — correct handling of malformed/large PDF files
- **Great CLI ecosystem** — `clap`, `indicatif`, `anyhow` are production-grade
- **Tauri-native** — Tauri's backend is Rust, so the core library integrates with no FFI overhead

### Shared Core Library Architecture

The most important architectural decision: all PDF logic lives in `ezpdf-core`, a Rust library crate.
Neither the CLI nor the GUI implements PDF operations directly — they are thin shells.

```
ezpdf-core/      ← pure Rust library, all PDF logic, zero I/O coupling
ezpdf-cli/       ← clap CLI, calls core, handles stdin/stdout/progress
ezpdf-app/       ← Tauri app: Rust commands (thin wrappers over core) + web UI
```

This means:
- Feature parity between CLI and GUI is guaranteed by design
- The core can be tested independently of any UI
- A future scripting API or REST API could be added as a 4th crate

### PDF Library: `lopdf`

Key requirement: **lossless quality preservation**. This means manipulating PDF structure directly
— moving page dictionaries and their referenced XObjects — without ever decoding/re-encoding content
streams. `lopdf` operates at the PDF object level and supports this pattern.

Alternatives considered:
- `pdfium-render`: Google's PDFium bindings — comprehensive but adds a large C++ dependency and
  can re-render, which risks quality loss.
- `pdf-rs`: Newer, more idiomatic, but less battle-tested for write operations.

`lopdf` is the pragmatic choice: active maintenance, proven write support, pure Rust.

### GUI: Tauri

Tauri uses the OS WebView (WebKit on macOS, WebKitGTK on Linux) — not bundled Chromium. App bundle
stays ~10-30MB vs 150MB+ for Electron. The frontend can be any web stack (React/Svelte/plain HTML).
The Rust backend calls `ezpdf-core` directly with zero overhead.

### Distribution

| Artifact | Install method |
|----------|----------------|
| CLI binary | `brew install ez/tap/ezpdf` |
| CLI binary (Rust devs) | `cargo install ezpdf` |
| Desktop `.app` / `.deb` | `brew install --cask ezpdf` / GitHub Releases |

## Key Decisions

- **Tool name**: `ezpdf` — EZ = owner's initials. CLI binary and app share the same name.
- **Workspace layout**: Cargo workspace with 3 crates: `ezpdf-core`, `ezpdf-cli`, `ezpdf-app`.
- **Page indexing**: 1-based (not 0-based) — matches how users think about PDF pages.
- **In-place edits**: Disallowed by default. Always write to `-o output.pdf`. Require `--in-place` to overwrite input.
- **Progress output**: Spinner/progress bar for large files (via `indicatif`), suppress with `--quiet`.
- **Error messages**: Show exact page count of the PDF when a user specifies an out-of-range page.
- **Shell completions**: Generate for bash/zsh/fish via `clap_complete` — `ezpdf completions zsh >> ~/.zshrc`.
- **Man pages**: Generated from clap definitions via `clap_mangen` — included in Homebrew formula.
- **Versioning**: Semantic versioning. Single `CHANGELOG.md` covers all crates.
- **Encrypted PDFs**: v1 detects them and exits with a clear error. Decrypt support deferred to v2.

## Performance Considerations

- **Streaming where possible**: Avoid loading entire PDFs into memory for metadata-only operations.
- **Parallel merge**: Use `rayon` to load/parse input files in parallel before combining.
- **Benchmark suite**: `criterion` benchmarks for core operations against large PDFs (100+ pages,
  50MB+) — regressions are caught before release.
- **GUI responsiveness**: Tauri's async commands ensure PDF operations never block the UI thread.

## Project Structure (Cargo Workspace)

```
ezpdf/
├── Cargo.toml                 # workspace definition
├── ezpdf-core/                # library crate — all PDF logic
│   ├── src/
│   │   ├── lib.rs
│   │   ├── merge.rs
│   │   ├── split.rs
│   │   ├── remove.rs
│   │   ├── rotate.rs
│   │   ├── reorder.rs
│   │   ├── page_range.rs      # "1-5,7,9-12" parser
│   │   └── error.rs
│   └── tests/
│       └── fixtures/          # sample PDFs
├── ezpdf-cli/                 # binary crate — CLI
│   └── src/
│       ├── main.rs
│       └── commands/          # thin clap subcommand handlers
├── ezpdf-app/                 # Tauri desktop app (v2)
│   ├── src-tauri/             # Rust backend
│   │   └── src/
│   │       ├── main.rs
│   │       └── commands.rs    # Tauri commands → ezpdf-core calls
│   └── src/                   # web frontend (React or Svelte)
├── benches/
│   └── large_pdf.rs           # criterion benchmarks
├── Formula/
│   └── ezpdf.rb               # Homebrew formula
├── CHANGELOG.md
└── README.md
```

## Open Questions

1. ~~**Tool name**~~: Resolved — `ezpdf` (EZ = owner's initials).
2. ~~**Encryption support**~~: Resolved — v1 detects encrypted PDFs and exits with a clear error. Deferred to v2.
3. ~~**Batch operations**~~: Resolved — deferred to v1.1. v1 operates on single files only.
4. ~~**Desktop app frontend**~~: Resolved — Svelte. Small bundle, no virtual DOM, ideal for a focused utility app.

## Next Steps

→ `/ce:plan` for full implementation plan: crate setup, dependency decisions, CI/CD (GitHub Actions
cross-compilation for macOS + Linux), Homebrew formula, and v1 milestone scope.
