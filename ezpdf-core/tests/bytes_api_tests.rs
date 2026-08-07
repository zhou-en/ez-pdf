//! The `*_bytes` API must be byte-for-byte identical to the path API.
//!
//! The path functions are thin wrappers over these, so any divergence means the
//! wasm build and the CLI would silently produce different output.

use ezpdf_core::error::EzPdfError;
use ezpdf_core::{
    info_bytes, merge_bytes, page_count_bytes, remove_bytes, rotate_bytes, split_range_bytes,
    to_markdown_bytes, MarkdownOptions,
};
use std::path::PathBuf;
use tempfile::{NamedTempFile, TempDir};

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn read_fixture(name: &str) -> Vec<u8> {
    std::fs::read(fixture(name)).expect("read fixture")
}

/// Runs a path-based op into a temp file and returns the bytes it wrote.
fn via_path<F>(op: F) -> Vec<u8>
where
    F: FnOnce(&std::path::Path),
{
    let out = NamedTempFile::new().unwrap();
    op(out.path());
    std::fs::read(out.path()).expect("read path-api output")
}

#[test]
fn merge_bytes_matches_path_api() {
    let a = read_fixture("3page.pdf");
    let b = read_fixture("5page.pdf");

    let from_bytes = merge_bytes(&[&a, &b]).unwrap();
    let from_path = via_path(|out| {
        ezpdf_core::merge(&[&fixture("3page.pdf"), &fixture("5page.pdf")], out).unwrap()
    });

    assert_eq!(from_bytes, from_path);
    assert_eq!(page_count_bytes(&from_bytes).unwrap(), 8);
}

#[test]
fn split_range_bytes_matches_path_api() {
    let input = read_fixture("5page.pdf");

    let from_bytes = split_range_bytes(&input, "2-4").unwrap();
    let from_path =
        via_path(|out| ezpdf_core::split_range(&fixture("5page.pdf"), "2-4", out).unwrap());

    assert_eq!(from_bytes, from_path);
    assert_eq!(page_count_bytes(&from_bytes).unwrap(), 3);
}

#[test]
fn remove_bytes_matches_path_api() {
    let input = read_fixture("5page.pdf");

    let from_bytes = remove_bytes(&input, "2,4").unwrap();
    let from_path = via_path(|out| ezpdf_core::remove(&fixture("5page.pdf"), "2,4", out).unwrap());

    assert_eq!(from_bytes, from_path);
    assert_eq!(page_count_bytes(&from_bytes).unwrap(), 3);
}

#[test]
fn rotate_bytes_matches_path_api() {
    let input = read_fixture("3page.pdf");

    let from_bytes = rotate_bytes(&input, 90, None).unwrap();
    let from_path =
        via_path(|out| ezpdf_core::rotate(&fixture("3page.pdf"), 90, None, out).unwrap());

    assert_eq!(from_bytes, from_path);
}

#[test]
fn rotate_bytes_with_page_selection_matches_path_api() {
    let input = read_fixture("5page.pdf");

    let from_bytes = rotate_bytes(&input, 180, Some("1,3")).unwrap();
    let from_path =
        via_path(|out| ezpdf_core::rotate(&fixture("5page.pdf"), 180, Some("1,3"), out).unwrap());

    assert_eq!(from_bytes, from_path);
}

#[test]
fn info_bytes_matches_path_api() {
    let input = read_fixture("3page.pdf");
    assert_eq!(
        info_bytes(&input).unwrap(),
        ezpdf_core::info(&fixture("3page.pdf")).unwrap()
    );
}

#[test]
fn page_count_bytes_matches_path_api() {
    let input = read_fixture("5page.pdf");
    assert_eq!(
        page_count_bytes(&input).unwrap(),
        ezpdf_core::page_count(&fixture("5page.pdf")).unwrap()
    );
}

#[test]
fn to_markdown_bytes_matches_path_api() {
    let input = read_fixture("text.pdf");
    let options = MarkdownOptions::default();
    assert_eq!(
        to_markdown_bytes(&input, &options).unwrap(),
        ezpdf_core::to_markdown(&fixture("text.pdf"), &options).unwrap()
    );
}

// ── Error parity ─────────────────────────────────────────────────────────────

#[test]
fn bytes_api_reports_encrypted_pdf() {
    let input = read_fixture("encrypted.pdf");
    assert!(matches!(
        page_count_bytes(&input),
        Err(EzPdfError::EncryptedPdf)
    ));
    assert!(matches!(
        rotate_bytes(&input, 90, None),
        Err(EzPdfError::EncryptedPdf)
    ));
}

#[test]
fn bytes_api_rejects_out_of_range_pages() {
    let input = read_fixture("3page.pdf");
    assert!(matches!(
        split_range_bytes(&input, "9"),
        Err(EzPdfError::PageOutOfRange { page: 9, total: 3 })
    ));
}

#[test]
fn remove_bytes_rejects_removing_every_page() {
    let input = read_fixture("3page.pdf");
    assert!(matches!(
        remove_bytes(&input, "1-3"),
        Err(EzPdfError::InvalidSyntax { .. })
    ));
}

#[test]
fn rotate_bytes_rejects_non_multiple_of_90() {
    let input = read_fixture("3page.pdf");
    assert!(matches!(
        rotate_bytes(&input, 45, None),
        Err(EzPdfError::InvalidSyntax { .. })
    ));
}

#[test]
fn bytes_api_rejects_garbage_input() {
    assert!(page_count_bytes(b"not a pdf at all").is_err());
    assert!(merge_bytes(&[b"garbage"]).is_err());
}

#[test]
fn merge_bytes_requires_at_least_one_input() {
    // Guard at the boundary: an empty merge would otherwise produce a
    // structurally valid but pageless PDF.
    assert!(merge_bytes(&[]).is_err());
}

/// `split_each` fans out to many files, so its bytes counterpart returns the
/// pages instead of writing them.
#[test]
fn split_each_bytes_returns_one_document_per_page() {
    let input = read_fixture("3page.pdf");
    let pages = ezpdf_core::split_each_bytes(&input).unwrap();

    assert_eq!(pages.len(), 3);
    for page in &pages {
        assert_eq!(page_count_bytes(page).unwrap(), 1);
    }

    // And each one matches what the path API burst into that directory.
    let dir = TempDir::new().unwrap();
    ezpdf_core::split_each(&fixture("3page.pdf"), dir.path()).unwrap();
    for (i, page) in pages.iter().enumerate() {
        let path = dir.path().join(format!("page-{}.pdf", i + 1));
        assert_eq!(
            *page,
            std::fs::read(&path).unwrap(),
            "page {} differs",
            i + 1
        );
    }
}
