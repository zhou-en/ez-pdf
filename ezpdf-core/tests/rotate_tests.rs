mod common;

use ezpdf_core::{error::EzPdfError, rotate};
use lopdf::Document;
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn get_page_rotation(path: &Path, page_num: u32) -> i64 {
    let doc = Document::load(path).expect("load PDF");
    let pages = doc.get_pages();
    let page_id = *pages.get(&page_num).expect("page not found");
    let page = doc.get_object(page_id).expect("get page object");
    let dict = page.as_dict().expect("page is dict");
    dict.get(b"Rotate")
        .and_then(|r| r.as_i64())
        .unwrap_or(0)
}

#[test]
fn rotate_all_pages_90_degrees() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    rotate(input.as_path(), 90, None, out.path()).unwrap();

    for page in 1..=3 {
        assert_eq!(get_page_rotation(out.path(), page), 90, "page {page}");
    }
}

#[test]
fn rotate_specific_pages_only() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    rotate(input.as_path(), 90, Some("1,3"), out.path()).unwrap();

    assert_eq!(get_page_rotation(out.path(), 1), 90);
    assert_eq!(get_page_rotation(out.path(), 2), 0, "page 2 should not be rotated");
    assert_eq!(get_page_rotation(out.path(), 3), 90);
}

#[test]
fn rotate_negative_90_same_as_270() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    rotate(input.as_path(), -90, None, out.path()).unwrap();

    assert_eq!(get_page_rotation(out.path(), 1), 270);
}

#[test]
fn rotate_180() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    rotate(input.as_path(), 180, None, out.path()).unwrap();

    assert_eq!(get_page_rotation(out.path(), 1), 180);
}

#[test]
fn rotate_invalid_degrees_returns_error() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    let result = rotate(input.as_path(), 45, None, out.path());

    assert!(
        matches!(result, Err(EzPdfError::InvalidSyntax { .. })),
        "expected InvalidSyntax for 45°, got: {:?}",
        result
    );
}

#[test]
fn rotate_round_trip() {
    let input = fixture("3page.pdf");
    let mid = NamedTempFile::new().unwrap();
    let out = NamedTempFile::new().unwrap();

    rotate(input.as_path(), 90, None, mid.path()).unwrap();
    rotate(mid.path(), -90, None, out.path()).unwrap();

    // Net rotation = 0 (back to original)
    assert_eq!(get_page_rotation(out.path(), 1), 0);
}
