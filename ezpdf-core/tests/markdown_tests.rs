mod common;

use ezpdf_core::error::EzPdfError;
use ezpdf_core::{markdown, to_markdown, MarkdownOptions};
use std::path::PathBuf;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn to_markdown_extracts_body_text() {
    let md = to_markdown(&fixture("text.pdf"), &MarkdownOptions::default()).unwrap();
    assert!(
        md.contains("The quick brown fox"),
        "body text missing from:\n{md}"
    );
}

#[test]
fn to_markdown_emits_heading_for_large_font() {
    let md = to_markdown(&fixture("text.pdf"), &MarkdownOptions::default()).unwrap();
    assert!(
        md.lines()
            .any(|l| l.starts_with('#') && l.contains("Quarterly Report")),
        "no heading line found in:\n{md}"
    );
}

#[test]
fn to_markdown_page_breaks_separate_pages() {
    let md = to_markdown(&fixture("text.pdf"), &MarkdownOptions::default()).unwrap();
    let separators = md.lines().filter(|l| l.trim() == "---").count();
    // text.pdf has 3 pages -> 2 separators, and never a leading one (a leading
    // `---` would be parsed as YAML frontmatter by Obsidian/Jekyll/Hugo).
    assert_eq!(separators, 2, "expected 2 separators in:\n{md}");
    assert!(!md.trim_start().starts_with("---"), "leading --- in:\n{md}");
}

#[test]
fn to_markdown_no_page_breaks_omits_separator() {
    let options = MarkdownOptions {
        page_breaks: false,
        ..Default::default()
    };
    let md = to_markdown(&fixture("text.pdf"), &options).unwrap();
    assert!(
        !md.lines().any(|l| l.trim() == "---"),
        "separator present in:\n{md}"
    );
}

#[test]
fn to_markdown_respects_page_range() {
    let options = MarkdownOptions {
        pages: Some("1".to_string()),
        ..Default::default()
    };
    let md = to_markdown(&fixture("text.pdf"), &options).unwrap();
    assert!(
        md.contains("Quarterly Report"),
        "page 1 text missing:\n{md}"
    );
    assert!(
        !md.contains("Page Three Heading"),
        "page 3 leaked in:\n{md}"
    );
}

#[test]
fn to_markdown_invalid_range_returns_error() {
    let options = MarkdownOptions {
        pages: Some("9".to_string()),
        ..Default::default()
    };
    let result = to_markdown(&fixture("text.pdf"), &options);
    assert!(matches!(
        result,
        Err(EzPdfError::PageOutOfRange { page: 9, total: 3 })
    ));
}

#[test]
fn to_markdown_empty_content_returns_no_text_layer() {
    let result = to_markdown(&fixture("5page.pdf"), &MarkdownOptions::default());
    match result {
        Err(EzPdfError::NoTextLayer { pages }) => assert_eq!(pages, vec![1, 2, 3, 4, 5]),
        other => panic!("expected NoTextLayer, got {other:?}"),
    }
}

#[test]
fn no_text_layer_error_names_a_remedy() {
    let err = EzPdfError::NoTextLayer { pages: vec![1] };
    let msg = err.to_string();
    assert!(msg.contains("ocrmypdf"), "no remedy in message: {msg}");
}

#[test]
fn to_markdown_encrypted_returns_encrypted_error() {
    let result = to_markdown(&fixture("encrypted.pdf"), &MarkdownOptions::default());
    assert!(matches!(result, Err(EzPdfError::EncryptedPdf)));
}

#[test]
fn to_markdown_missing_file_returns_io_error() {
    let result = to_markdown(&fixture("does_not_exist.pdf"), &MarkdownOptions::default());
    assert!(matches!(result, Err(EzPdfError::Io(_))));
}

#[test]
fn markdown_writes_file_matching_to_markdown() {
    let out = NamedTempFile::new().unwrap();
    let options = MarkdownOptions::default();
    markdown(&fixture("text.pdf"), &options, out.path()).unwrap();

    let written = std::fs::read_to_string(out.path()).unwrap();
    let expected = to_markdown(&fixture("text.pdf"), &options).unwrap();
    assert_eq!(written, expected);
    assert!(!written.trim().is_empty());
}

#[allow(dead_code)]
fn _shape_check(o: MarkdownOptions) {
    let _: Option<String> = o.pages;
    let _: bool = o.page_breaks;
}
