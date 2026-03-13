mod common;

use common::create_test_pdf;
use ezpdf_core::{error::EzPdfError, reorder};
use lopdf::Document;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Creates a labeled 3-page PDF where each page has a distinguishable label.
fn labeled_3page_pdf() -> NamedTempFile {
    // We use the standard fixture for structure testing; page order is
    // verified by checking the /Kids array order in the Pages tree.
    let bytes = create_test_pdf(3, "labeled-3page");
    let mut tmp = NamedTempFile::new().unwrap();
    tmp.write_all(&bytes).unwrap();
    tmp
}

fn get_kids_order(path: &Path) -> Vec<lopdf::ObjectId> {
    let doc = Document::load(path).expect("load PDF");
    let catalog = doc.catalog().expect("catalog");
    let pages_id = catalog.get(b"Pages").unwrap().as_reference().unwrap();
    let pages = doc
        .get_object(pages_id)
        .unwrap()
        .as_dict()
        .unwrap();
    pages
        .get(b"Kids")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|o| o.as_reference().unwrap())
        .collect()
}

#[test]
fn reorder_3_page_changes_kids_order() {
    let src = labeled_3page_pdf();
    let original_kids = get_kids_order(src.path());

    let out = NamedTempFile::new().unwrap();
    reorder(src.path(), "3,1,2", out.path()).unwrap();

    let reordered_kids = get_kids_order(out.path());

    // The Kids array should now be [page3, page1, page2] from the original IDs
    assert_eq!(reordered_kids[0], original_kids[2], "first kid should be original page 3");
    assert_eq!(reordered_kids[1], original_kids[0], "second kid should be original page 1");
    assert_eq!(reordered_kids[2], original_kids[1], "third kid should be original page 2");
}

#[test]
fn reorder_round_trip_restores_original_order() {
    let src = labeled_3page_pdf();
    let original_kids = get_kids_order(src.path());

    let mid = NamedTempFile::new().unwrap();
    reorder(src.path(), "2,1", mid.path()).unwrap();

    let out = NamedTempFile::new().unwrap();
    reorder(mid.path(), "2,1", out.path()).unwrap();

    let final_kids = get_kids_order(out.path());
    // After two "2,1" reorders, order should be restored
    assert_eq!(final_kids, original_kids);
}

#[test]
fn reorder_missing_page_returns_error() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    let result = reorder(input.as_path(), "1,2", out.path());

    assert!(
        matches!(result, Err(EzPdfError::InvalidSyntax { .. })),
        "expected error for incomplete order, got: {:?}",
        result
    );
}

#[test]
fn reorder_duplicate_page_returns_error() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    let result = reorder(input.as_path(), "1,1,2,3", out.path());

    assert!(
        matches!(result, Err(EzPdfError::InvalidSyntax { .. })),
        "expected error for duplicate page, got: {:?}",
        result
    );
}

#[test]
fn reorder_out_of_range_page_returns_error() {
    let input = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    let result = reorder(input.as_path(), "1,2,5", out.path());

    assert!(
        matches!(
            result,
            Err(EzPdfError::PageOutOfRange { .. }) | Err(EzPdfError::InvalidSyntax { .. })
        ),
        "expected out-of-range error, got: {:?}",
        result
    );
}
