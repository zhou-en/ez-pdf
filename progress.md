# ezpdf — Implementation Progress

## 2026-03-12 — Phase 1 complete: Project Foundation

**Completed tasks:**
- 1.1 Cargo workspace with ezpdf-core (lib), ezpdf-cli (bin), ezpdf-app (placeholder)
- 1.2 GitHub Actions CI workflow (ubuntu-latest + macos-latest)
- 1.3 rustfmt.toml, clippy.toml, .gitignore, LICENSE (MIT), CHANGELOG.md
- 1.4 Review — all checks pass

**Tests passing:** 0 (infrastructure only — no logic yet)

**Deviations / blockers found:**
- none

## 2026-03-12 — Phase 2 complete: Page Range Parser

**Completed tasks:**
- 2.1 [RED] 15 failing tests for page_range::parse + 2 EzPdfError display tests
- 2.2 [GREEN] parse() + parse_segment() + parse_page_number() + check_in_range() helpers
- 2.3 [REFACTOR] extracted helpers, fixed clippy::manual_strip warning
- 2.4 [REVIEW] 17/17 tests pass, clippy clean, fmt clean

**Tests passing:** 17

**Deviations / blockers found:**
- none

## 2026-03-12 — Phase 3 complete: Merge Command

**Completed tasks:**
- 3.1 [SETUP] Fixture generator (create_test_pdf), 3page.pdf + 5page.pdf fixtures committed
- 3.2 [RED] 4 failing tests for merge (page sum, 3-way merge, missing input, bad output dir)
- 3.3 [GREEN] merge() impl: renumbers objects per doc, builds fresh /Pages tree, save_to()
- 3.4 [RED] 2 failing CLI tests (merge exits 0 + "Merged", error exits 1 + "Error:")
- 3.5 [GREEN] ezpdf merge subcommand wired with MergeArgs, print_success helper
- 3.6 [REFACTOR] clippy clean (io_other_error), quiet flag, output module extracted
- 3.7 [REVIEW] 23 tests pass, manual demo ✓

**Tests passing:** 23

**Deviations / blockers found:**
- encrypted PDF test deferred to Phase 8 (needs qpdf fixture, fully covered there)

## 2026-03-12 — Phase 4 complete: Remove Command

**Completed tasks:**
- 4.1 [RED] 5 failing tests (remove middle, first+last, range, all pages, out-of-range)
- 4.2 [GREEN] remove() = parse pages_to_remove, compute keep list, rebuild /Pages tree
- 4.3 [RED] 2 failing CLI tests
- 4.4 [GREEN] ezpdf remove subcommand wired
- 4.5 [REFACTOR] fmt + clippy clean, "cannot remove all N pages" msg with count context
- 4.6 [REVIEW] 30 tests pass, clippy/fmt clean

**Tests passing:** 30

**Deviations / blockers found:**
- none

## 2026-03-12 — Phase 5 complete: Split Command

**Completed tasks:**
- 5.1 [RED] 6 failing tests (split_range 2 tests, split_each 4 tests)
- 5.2 [GREEN] split_range/split_each via build_kept + digits_needed for zero-padding
- 5.3 [RED] 2 failing CLI tests (range mode + --each mode)
- 5.4 [GREEN] ezpdf split subcommand, range vs --each branching
- 5.5 [REFACTOR] fmt + clippy clean
- 5.6 [REVIEW] 38 tests pass

**Tests passing:** 38

**Deviations / blockers found:**
- none

## 2026-03-12 — Phase 6 complete: Rotate Command

**Completed tasks:**
- 6.1 [RED] 6 failing tests (rotate all, specific pages, -90=270, 180, invalid, round-trip)
- 6.2 [GREEN] rotate() edits /Rotate dict entry, normalized to 0/90/180/270 via rem_euclid
- 6.3 [RED] 2 failing CLI tests (rotate all, rotate --pages flag)
- 6.4 [GREEN] ezpdf rotate subcommand wired
- 6.5 [REFACTOR] fmt clean, normalize_degrees validates multiples of 90
- 6.6 [REVIEW] 46 tests pass, clippy/fmt clean

**Tests passing:** 46

**Deviations / blockers found:**
- none

## 2026-03-12 — Phase 7 complete: Reorder Command

**Completed tasks:**
- 7.1 [RED] 5 failing tests for reorder (page order change, round-trip, missing, duplicate, out-of-range)
- 7.2 [GREEN] reorder() + parse_order() with full validation; clippy::manual_contains fixed
- 7.3 [RED] 5 failing CLI tests for `ezpdf reorder`
- 7.4 [GREEN] ezpdf reorder subcommand wired with ReorderArgs
- 7.5 [REFACTOR] fmt + clippy clean; error messages include page details
- 7.6 [REVIEW] all tests pass, manual round-trip demo ✓

**Tests passing:** 56

**Deviations / blockers found:**
- none

## 2026-03-12 — Phase 8 complete: CLI Polish

**Completed tasks:**
- 8.1 [RED] 4 failing tests for version, unknown subcommand, zsh/bash completions
- 8.2 [GREEN] `completions` subcommand via clap_complete, `--version` already built-in
- 8.3 [SETUP] Progress bar helper (maybe_progress) wired into merge, remove, split-each for PDFs > 20 pages
- 8.4 [RED] 6 failing tests for encrypted PDF detection (all 5 commands) + missing file path
- 8.5 [GREEN] `load_doc` adds `is_encrypted()` check + path in IO error messages; encrypted.pdf fixture
- 8.6 [REFACTOR] fmt + clippy clean, long_about with examples on root command
- 8.7 [REVIEW] all tests pass, manual demo ✓

**Tests passing:** 66

**Deviations / blockers found:**
- qpdf not available on dev machine; encrypted fixture created via lopdf stub (/Encrypt in trailer)

## 2026-03-12 — Phase 9 complete: Performance & Benchmarks

**Completed tasks:**
- 9.1 [SETUP] criterion benchmarks for merge, split_each, remove, rotate; baseline recorded
- 9.2 [SETUP] rayon parallel file loading in merge (par_iter)
- 9.3 [REVIEW] baseline.md committed, all tests pass

**Benchmarks (Apple M3, release build):**
- merge 5×10-page: ~9.8 ms (parallel)
- split_each 50-page: ~333 ms (I/O bound)
- remove 25/50 pages: ~8.1 ms
- rotate 50 pages: ~8.0 ms

**Deviations / blockers found:**
- none

## 2026-03-12 — Phase 10 complete: Distribution & Release

**Completed tasks:**
- 10.1 [SETUP] `.github/workflows/release.yml` — 4-target matrix (macOS arm64, macOS x86, Linux x86, Linux arm64 via cross)
- 10.4 [SETUP] Cargo.toml metadata verified: description, repository, keywords, categories all set
- 10.5 [DONE] README.md comprehensive with all 5 commands, install, dev guide
- 10.6 [REVIEW] All tests pass, clippy/fmt clean

**Tests passing:** 66 (all phases)

**Deviations / blockers found:**
- Homebrew tap + crates.io publish require an active GitHub release
- To release: `git tag v0.1.0 && git push origin main --tags`

---

## EZPDF V1 COMPLETE

All 10 phases done. 66 tests passing. Clippy/fmt clean.

---

## 2026-03-13 — Backlog planning complete: Phases 11–20 added

**Planned phases:**
- Phase 11: `ezpdf info` command (v1.1)
- Phase 12: Batch operations `--batch` flag (v1.1)
- Phase 13: PDF metadata read/write (v1.2)
- Phase 14: Windows support (v1.2)
- Phase 15: Encrypted PDF `--password` support (v2)
- Phase 16: Watermark pages (v2)
- Phase 17: Bookmarks / outline manipulation (v2)
- Phase 18: Image extraction (v2)
- Phase 19: PDF optimization (v2)
- Phase 20: Desktop app — Tauri v2 + Svelte 5 (v2)

**Tests passing:** 66 (no new tests yet — backlog phases all unchecked)

**Deviations / blockers found:**
- none

---

## 2026-03-13 — Phase 11 complete: `ezpdf info` Command

**Completed tasks:**
- 11.1 [RED] failing tests for `info()` and `PdfInfo` struct
- 11.2 [GREEN] `ezpdf-core/src/info.rs` — page count, dimensions, metadata
- 11.3 [RED] failing CLI tests for `ezpdf info`
- 11.4 [GREEN] `ezpdf-cli/src/commands/info.rs` — normal + --json output
- 11.5 [REFACTOR] paper size labels, --pages flag, serde on PdfInfo
- 11.6 [REVIEW] all tests pass, clippy clean

**Tests passing:** 73 (7 new)

**Deviations / blockers found:**
- none

---

## 2026-03-13 — Phase 12 complete: Batch Operations

**Completed tasks:**
- 12.1 [RED] failing tests for `collect_pdf_inputs`
- 12.2 [GREEN] `ezpdf-core/src/batch.rs` — collect_pdf_inputs
- 12.3 [RED] failing CLI tests for --batch on rotate, remove, merge
- 12.4 [GREEN] --batch flag on all 5 commands; merge collects dir inputs; others use run_batch_independent
- 12.5 [REFACTOR] extracted `run_batch_independent` helper in output.rs; rotate/remove/reorder use it
- 12.6 [REVIEW] all tests pass, clippy clean

**Tests passing:** 81 (8 new)

**Deviations / blockers found:**
- none

---

## 2026-03-13 — Phase 13 complete: PDF Metadata Read/Write

**Completed tasks:**
- 13.1 [RED] failing tests for `get_metadata` and `set_metadata`; defined `PdfMetadata` and `MetadataUpdate` structs
- 13.2 [GREEN] `ezpdf-core/src/metadata.rs` — get_metadata reads Info dict; set_metadata creates/updates Info dict with clear_all support
- 13.3 [RED] failing CLI tests for `ezpdf meta get` and `ezpdf meta set`
- 13.4 [GREEN] `ezpdf-cli/src/commands/meta.rs` — nested `get`/`set` subcommands with `--json` and `--title/--author/...` flags
- 13.5 [REFACTOR] aligned key: value output, Serialize on PdfMetadata, clippy clean
- 13.6 [REVIEW] all tests pass, clippy clean

**Tests passing:** 89 (8 new)

**Deviations / blockers found:**
- none

---

## 2026-03-14 — Phase 15 complete: Encrypted PDF Support

**Completed tasks:**
- 15.1 [SETUP] Researched lopdf 0.31: native `doc.decrypt(password)` available — no qpdf shell-out needed. Wrong password → `DecryptionError::IncorrectPassword`. Created `encrypted_pw.pdf` fixture via qpdf (password: "secret").
- 15.2 [RED] Added `WrongPassword` to `EzPdfError`; wrote 3 failing tests in `load_tests.rs`
- 15.3 [GREEN] `load_doc_with_password` in `merge.rs`; `load_doc` delegates to it with `None`
- 15.4 [RED] Failing CLI tests for `--password` on merge/rotate/remove/reorder/split
- 15.5 [GREEN] `resolve_input` helper in `output.rs` pre-decrypts to temp file; added `--password` to all 5 Args
- 15.6 [REFACTOR] Added `--password-file` flag; `resolve_password` helper; clippy clean
- 15.7 [REVIEW] all tests pass, clippy clean

**Tests passing:** 97 (8 new)

**Deviations / blockers found:**
- Used pre-decrypt-to-tempfile approach in CLI instead of threading password through core function signatures — avoids breaking changes to core API and is simpler overall
