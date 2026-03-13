mod common;

use ezpdf_core::{error::EzPdfError, remove};
use lopdf::Document;
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn page_count(path: &Path) -> u32 {
    Document::load(path).expect("load output PDF").get_pages().len() as u32
}

#[test]
fn remove_middle_page_from_5_page_pdf() {
    let input = fixture("5page.pdf");
    let out = NamedTempFile::new().unwrap();

    remove(input.as_path(), "3", out.path()).unwrap();

    assert_eq!(page_count(out.path()), 4);
}

#[test]
fn remove_first_and_last_from_5_page_pdf() {
    let input = fixture("5page.pdf");
    let out = NamedTempFile::new().unwrap();

    remove(input.as_path(), "1,5", out.path()).unwrap();

    assert_eq!(page_count(out.path()), 3);
}

#[test]
fn remove_range_from_5_page_pdf() {
    let input = fixture("5page.pdf");
    let out = NamedTempFile::new().unwrap();

    remove(input.as_path(), "2-4", out.path()).unwrap();

    assert_eq!(page_count(out.path()), 2);
}

#[test]
fn remove_all_pages_returns_error() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    let result = remove(input.as_path(), "1-3", out.path());

    assert!(
        matches!(result, Err(EzPdfError::InvalidSyntax { .. })),
        "expected error when removing all pages, got: {:?}",
        result
    );
}

#[test]
fn remove_out_of_range_page_returns_error() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    let result = remove(input.as_path(), "5", out.path());

    assert!(
        matches!(result, Err(EzPdfError::PageOutOfRange { .. })),
        "expected PageOutOfRange, got: {:?}",
        result
    );
}
