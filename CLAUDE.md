# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
# Full check — what CI runs (ci.yml), in order
cargo test --workspace
cargo clippy --workspace -- -D warnings
cargo fmt --check

# Single crate / single test
cargo test -p ezpdf-core
cargo test -p ezpdf-core --test merge_tests            # one integration test file
cargo test -p ezpdf-core page_range::tests::single_page # one unit test
cargo test -p ezpdf-cli --test split_cli_tests

# Benchmarks (criterion, ezpdf-core only)
cargo bench -p ezpdf-core

# Regenerate committed test fixtures (normally never needed — they're in git)
cargo test -p ezpdf-core --test generate_fixtures -- --ignored

# Desktop app frontend (pnpm, from ezpdf-app/frontend)
pnpm install
pnpm test          # vitest run
pnpm test:watch

# Desktop app dev / bundle (from ezpdf-app)
./frontend/node_modules/.bin/tauri dev
./frontend/node_modules/.bin/tauri build
```

Building anything in the workspace on Linux requires the Tauri GTK stack, because `ezpdf-app` is a workspace member and `--workspace` compiles it:

```bash
sudo apt-get install -y libglib2.0-dev libgtk-3-dev libwebkit2gtk-4.1-dev \
  libappindicator3-dev librsvg2-dev libsoup-3.0-dev libjavascriptcoregtk-4.1-dev patchelf
```

To skip that, scope commands with `-p ezpdf-core -p ezpdf-cli`.

## Architecture

Cargo workspace, three crates. **All PDF logic lives in `ezpdf-core`; `ezpdf-cli` and `ezpdf-app` are thin shells that parse input and call it.** New functionality goes in core first, then gets exposed by each shell.

- **`ezpdf-core`** — library over the `lopdf` object model. One module per operation (`merge`, `split`, `remove`, `rotate`, `reorder`, `metadata`, `watermark`, `bookmarks`, `images`, `info`, `optimize`), plus `page_range` (the `1-5,7,9-` parser), `batch` (directory globbing), and `error`. Public API is re-exported flat from `lib.rs`.
- **`ezpdf-cli`** — clap binary named `ezpdf`. `main.rs` maps each subcommand to `commands/<name>.rs`, which each expose `Args` + `run(args) -> anyhow::Result<()>`. Shared helpers live in `output.rs`.
- **`ezpdf-app`** — Tauri v2 shell. `src/commands.rs` holds `#[tauri::command]` wrappers (one per core op, `String` in / `String` out, errors flattened with `.map_err(|e| e.to_string())`); `src/lib.rs` registers them in `invoke_handler!`. Frontend is Svelte 5 + Vite in `frontend/`, tested with Vitest + `@testing-library/svelte`.

### Core invariants

- **Lossless is the product.** Operations manipulate the PDF object graph only — copy page object refs, edit page dictionaries, rewrite the `/Kids` array. Never decode or re-encode content streams or images. Any change that would re-render a page is wrong.
- **All fallible core paths return `EzPdfError`** (`error.rs`). No `unwrap()` / `expect()` / panics in library code; CLI code converts to `anyhow` at the boundary. Error messages are user-facing and carry a remedy (e.g. `EncryptedPdf` names the `qpdf --decrypt` fix).
- **Pages are 1-indexed everywhere**, including error messages; `0` is a syntax error.

### Shared primitives to reuse rather than reimplement

- `merge::load_doc(path)` / `load_doc_with_password(path, pw)` — the only way documents are opened. Maps `lopdf` errors into `EzPdfError`, and turns encryption into `EncryptedPdf` / `WrongPassword`.
- `remove::build_kept(doc, &[page_nums])` — builds a new document containing only the given pages, in the given order. `split_range`, `split_each`, and `remove` are all thin wrappers over it.
- `page_range::parse(input, page_count)` — every page-selecting flag goes through this; it owns range validation and out-of-range errors.
- CLI: `output::resolve_password`, `resolve_input` (decrypts to a `NamedTempFile` the caller must keep alive), `run_batch_independent`, `maybe_progress` (progress bar only above 20 pages), `print_success`.

### Batch is a flag, not a subcommand

Despite the README's command table, there is no `ezpdf batch`. Each operation takes `--batch`, which reinterprets its input arg as a directory (`ezpdf_core::batch::collect_pdf_inputs`). `merge --batch` folds a directory into one file; the others map over files independently into an output directory.

## Testing conventions

- Core: unit tests inline (`page_range.rs`), behaviour tests as integration files in `ezpdf-core/tests/*.rs` against committed fixtures in `tests/fixtures/`. `tests/common/mod.rs` builds PDFs programmatically — use it rather than adding binary fixtures where possible.
- CLI: `assert_cmd` + `predicates` end-to-end tests in `ezpdf-cli/tests/`, one file per command.
- App: Rust command tests live in `ezpdf-app/src/lib.rs`'s `mod tests` and load the core fixtures via a relative path; Svelte component tests sit next to their components as `*.test.ts`.

The project was built strictly TDD (see `PROMPT.md`, `task_plan.md`, `progress.md` — the autonomous ralph-loop harness). Keep to it: failing test first, minimum implementation, then refactor.

## Gotchas

- `beforeDevCommand` / `beforeBuildCommand` in `tauri.conf.json` are bare `pnpm dev` / `pnpm build` with no `cd`: the Tauri CLI already runs hooks from the resolved frontend directory (the nearest `package.json` below the invocation cwd, i.e. `ezpdf-app/frontend`). Don't reintroduce a `cd` or an absolute path — run `tauri` from `ezpdf-app` and the hooks resolve on any machine.
- `Cargo.lock` is gitignored, so dependency resolution can drift between checkouts.
- Frontend uses **pnpm** (there's a `pnpm-lock.yaml`); don't introduce npm/yarn lockfiles.
- `output/` and the loose `.pdf` / `tmux-*.log` files in the repo root are scratch artifacts, not sources.
