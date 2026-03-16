# ezpdf

A fast, lossless PDF manipulation tool written in Rust — available as a CLI and a native desktop app (macOS + Linux).

Merge, split, rotate, watermark, edit metadata, and more without ever re-encoding content. Built as a modern replacement for `pdftk`.

---

## Features

### CLI (`ezpdf`)

| Command      | Description                                          |
|--------------|------------------------------------------------------|
| `merge`      | Combine two or more PDFs into one                    |
| `split`      | Extract a page range or burst into individual pages  |
| `remove`     | Delete specific pages or ranges                      |
| `rotate`     | Rotate all or specific pages by 90/180/270°          |
| `reorder`    | Rearrange pages by specifying a new order            |
| `info`       | Print page count, dimensions, and PDF metadata       |
| `meta`       | Read or write title, author, subject, keywords       |
| `watermark`  | Stamp diagonal text on all or specific pages         |
| `bookmarks`  | List or add bookmarks (PDF outline entries)          |
| `images`     | Extract embedded images to a folder                  |
| `batch`      | Run any operation across multiple files in parallel  |

### Desktop app (`ezpdf-app`)

All CLI operations are available in a drag-and-drop GUI built with Tauri v2 + Svelte 5:

- Drop PDFs onto the window; drag to reorder the merge list
- Switch operations from the sidebar (Merge, Split, Remove, Rotate, Reorder, Metadata, Watermark, Bookmarks, Extract)
- "Save As…" picker for every output path
- Dark mode (follows system preference)
- Progress bar while running
- Ships as `.dmg` (macOS) and `.deb` / `.AppImage` (Linux)

---

## Installation

### CLI

```bash
# Homebrew (macOS / Linux)
brew install zhou-en/tap/ezpdf

# Cargo
cargo install ezpdf
```

### Desktop app

Download the latest release from [GitHub Releases](https://github.com/zhou-en/ez-pdf/releases):

| Platform | Format |
|----------|--------|
| macOS    | `.dmg` |
| Linux    | `.deb` or `.AppImage` |

---

## Usage

```bash
# Merge PDFs
ezpdf merge a.pdf b.pdf c.pdf -o combined.pdf

# Split — extract a range
ezpdf split report.pdf 1-10 -o part1.pdf

# Split — burst into individual pages
ezpdf split report.pdf --each -o ./pages/

# Remove pages
ezpdf remove report.pdf 3,5,7-9 -o cleaned.pdf

# Rotate all pages
ezpdf rotate report.pdf 90 -o rotated.pdf

# Rotate specific pages
ezpdf rotate report.pdf 90 --pages 1,3,5 -o rotated.pdf

# Reorder pages
ezpdf reorder report.pdf 3,1,2,4 -o reordered.pdf

# Print file info
ezpdf info report.pdf

# Edit metadata
ezpdf meta report.pdf --title "Annual Report" --author "Alice" -o out.pdf

# Add a watermark
ezpdf watermark report.pdf "DRAFT" --opacity 0.3 -o out.pdf

# List bookmarks
ezpdf bookmarks list report.pdf

# Add a bookmark
ezpdf bookmarks add report.pdf "Chapter 1" --page 1 -o out.pdf

# Extract images
ezpdf images report.pdf -o ./images/

# Batch merge across a folder
ezpdf batch merge ./invoices/ -o combined.pdf

# Shell completions
ezpdf completions zsh >> ~/.zshrc
```

Page ranges use `1-5,7,9-12` syntax consistently across all commands. Pages are 1-indexed.

---

## Architecture

### Workspace Structure

`ezpdf` is a Cargo workspace with a shared core library. All PDF logic lives in `ezpdf-core` — the CLI and the future desktop app are thin shells over it.

```mermaid
%% Architecture: Cargo workspace crate relationships
flowchart TB
    user([User]):::startEnd
    cli(ezpdf-cli<br/>binary):::process
    app(ezpdf-app<br/>Tauri v2 + Svelte — v2):::process
    core(ezpdf-core<br/>library):::success
    lopdf(lopdf crate<br/>PDF object model):::process
    output([output.pdf]):::startEnd

    user --> cli
    user --> app
    cli --> core
    app --> core
    core --> lopdf
    lopdf --> output

    classDef startEnd fill:#e8d4f0,stroke:#333,stroke-width:2px,color:#333
    classDef process fill:#e8f4fd,stroke:#333,stroke-width:2px,color:#333
    classDef decision fill:#fff5e6,stroke:#333,stroke-width:2px,color:#333
    classDef error fill:#fde8e8,stroke:#333,stroke-width:2px,color:#333
    classDef success fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#333
```

### Command Execution Flow

Every command follows the same path: CLI parses args → routes to core function → lossless PDF manipulation → file written.

```mermaid
%% Command flow: CLI to core to output
flowchart LR
    A([ezpdf command]):::startEnd
    B(clap parses args):::process
    C{command?}:::decision
    D(merge.rs):::process
    E(split.rs):::process
    F(remove.rs):::process
    G(rotate.rs):::process
    H(reorder.rs):::process
    I(ezpdf-core):::success
    J(lopdf — object level edit):::process
    K{encrypted?}:::decision
    L(EzPdfError — suggest qpdf):::error
    M([output.pdf]):::startEnd

    A --> B --> C
    C -->|merge| D
    C -->|split| E
    C -->|remove| F
    C -->|rotate| G
    C -->|reorder| H
    D & E & F & G & H --> I
    I --> K
    K -->|no| J
    K -->|yes| L
    J --> M

    classDef startEnd fill:#e8d4f0,stroke:#333,stroke-width:2px,color:#333
    classDef process fill:#e8f4fd,stroke:#333,stroke-width:2px,color:#333
    classDef decision fill:#fff5e6,stroke:#333,stroke-width:2px,color:#333
    classDef error fill:#fde8e8,stroke:#333,stroke-width:2px,color:#333
    classDef success fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#333
```

### Lossless Quality Guarantee

`ezpdf` never re-renders pages. It works at the PDF object level — moving page dictionaries and their referenced content streams around without ever decoding them.

```mermaid
%% Lossless approach: what ezpdf does vs what it avoids
flowchart LR
    A([input.pdf]):::startEnd
    B(load PDF object graph):::process
    C{operation}:::decision
    D(copy page object refs):::success
    E(edit page dictionary):::success
    F(reorder Kids array):::success
    G(NEVER decode content streams):::error
    H(NEVER re-encode images):::error
    I(write modified object graph):::process
    J([output.pdf — same quality]):::startEnd

    A --> B --> C
    C -->|merge/split/remove| D
    C -->|rotate| E
    C -->|reorder| F
    D & E & F --> I
    G -.->|avoided| I
    H -.->|avoided| I
    I --> J

    classDef startEnd fill:#e8d4f0,stroke:#333,stroke-width:2px,color:#333
    classDef process fill:#e8f4fd,stroke:#333,stroke-width:2px,color:#333
    classDef decision fill:#fff5e6,stroke:#333,stroke-width:2px,color:#333
    classDef error fill:#fde8e8,stroke:#333,stroke-width:2px,color:#333
    classDef success fill:#e8f5e9,stroke:#333,stroke-width:2px,color:#333
```

---

## Project Structure

```
ez-pdf/
├── ezpdf-core/               # Library — all PDF logic
│   ├── src/
│   │   ├── merge.rs, split.rs, remove.rs, rotate.rs, reorder.rs
│   │   ├── metadata.rs       # read/write PDF info dictionary
│   │   ├── watermark.rs      # diagonal text stamp
│   │   ├── bookmarks.rs      # outline tree read/write
│   │   ├── images.rs         # embedded image extraction
│   │   ├── info.rs           # page count, dimensions
│   │   ├── batch.rs          # parallel multi-file operations
│   │   ├── optimize.rs       # stream compression
│   │   ├── page_range.rs     # "1-5,7,9-12" parser
│   │   └── error.rs
│   └── tests/
│       └── fixtures/         # sample PDFs for integration tests
├── ezpdf-cli/                # Binary — CLI (clap)
│   └── src/
│       ├── main.rs
│       └── commands/
├── ezpdf-app/                # Desktop app — Tauri v2 + Svelte 5
│   ├── src/                  # Rust Tauri commands
│   ├── frontend/src/         # Svelte components + Vitest tests
│   └── tauri.conf.json
├── docs/
│   ├── brainstorms/
│   └── plans/
└── .github/workflows/
    ├── ci.yml                # test + clippy on every PR
    └── release.yml           # CLI tarballs + .dmg + .deb + .AppImage on tag push
```

---

## Development

### Prerequisites

```bash
# Rust (stable)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Build & test
cargo build --workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

---

## Roadmap

| Version | Scope | Status |
|---------|-------|--------|
| **v1.0** | merge, split, remove, rotate, reorder — CLI + Homebrew | ✅ shipped |
| **v1.1** | batch operations, `ezpdf info` | ✅ shipped |
| **v2.0** | desktop app (Tauri v2 + Svelte 5) | ✅ shipped |
| **v2.1** | UX polish — drag-to-reorder, Save As, dark mode, progress bar | ✅ shipped |
| **v2.2** | metadata editor, watermark, bookmarks, image extraction in app UI | ✅ shipped |
| **v2.3** | Linux packaging — `.deb` + `.AppImage` via CI | ✅ shipped |

---

## License

MIT
