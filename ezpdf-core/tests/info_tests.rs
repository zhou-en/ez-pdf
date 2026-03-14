mod common;

use ezpdf_core::{error::EzPdfError, info, PdfInfo};
use std::path::Path;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn info_page_count_matches_fixture() {
    let path = fixture("3page.pdf");
    let result = info(path.as_path()).unwrap();
    assert_eq!(result.page_count, 3);
}

#[test]
fn info_dimensions_has_entry_per_page() {
    let path = fixture("3page.pdf");
    let result = info(path.as_path()).unwrap();
    assert_eq!(result.dimensions.len(), 3);
    for (w, h) in &result.dimensions {
        assert!(*w > 0.0, "width should be positive, got {w}");
        assert!(*h > 0.0, "height should be positive, got {h}");
    }
}

#[test]
fn info_missing_file_returns_io_error() {
    let path = Path::new("/tmp/does_not_exist_ezpdf_info.pdf");
    let result = info(path);
    assert!(
        matches!(result, Err(EzPdfError::Io(_))),
        "expected Io error, got: {:?}",
        result
    );
}

#[test]
fn info_encrypted_pdf_returns_encrypted_error() {
    let path = fixture("encrypted.pdf");
    let result = info(path.as_path());
    assert!(
        matches!(result, Err(EzPdfError::EncryptedPdf)),
        "expected EncryptedPdf error, got: {:?}",
        result
    );
}

// Compile-time shape check: PdfInfo must have all required fields
#[allow(dead_code)]
fn _shape_check(i: PdfInfo) {
    let _: u32 = i.page_count;
    let _: Vec<(f64, f64)> = i.dimensions;
    let _: Option<String> = i.title;
    let _: Option<String> = i.author;
    let _: Option<String> = i.subject;
    let _: Option<String> = i.keywords;
    let _: Option<String> = i.creator;
    let _: Option<String> = i.producer;
}
