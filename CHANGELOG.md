# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `ezpdf markdown` — convert a PDF to a Markdown file, with headings, lists and
  tables inferred from the document's layout. Available in the CLI (with
  `--pages`, `--no-page-breaks` and `--batch`) and in the desktop app.
- `EzPdfError::NoTextLayer` — raised for scanned/image-only PDFs, naming
  `ocrmypdf` as the remedy rather than emitting an empty file.
