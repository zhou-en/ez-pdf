mod common;

use ezpdf_core::{error::EzPdfError, merge};
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
fn merge_two_pdfs_page_count_is_sum() {
    let a = fixture("3page.pdf");
    let b = fixture("5page.pdf");
    let out = NamedTempFile::new().unwrap();

    merge(&[a.as_path(), b.as_path()], out.path()).unwrap();

    assert_eq!(page_count(out.path()), 8);
}

#[test]
fn merge_three_pdfs_correct_total() {
    let a = fixture("3page.pdf");
    let b = fixture("5page.pdf");
    let c = fixture("3page.pdf");
    let out = NamedTempFile::new().unwrap();

    merge(&[a.as_path(), b.as_path(), c.as_path()], out.path()).unwrap();

    assert_eq!(page_count(out.path()), 11);
}

#[test]
fn merge_missing_input_returns_io_error() {
    let missing = Path::new("/tmp/does_not_exist_ezpdf.pdf");
    let out = NamedTempFile::new().unwrap();

    let result = merge(&[missing], out.path());

    assert!(
        matches!(result, Err(EzPdfError::Io(_))),
        "expected Io error, got: {:?}",
        result
    );
}

#[test]
fn merge_output_to_nonexistent_directory_returns_io_error() {
    let a = fixture("3page.pdf");
    let out = Path::new("/tmp/no_such_dir_ezpdf/out.pdf");

    let result = merge(&[a.as_path()], out);

    assert!(
        matches!(result, Err(EzPdfError::Io(_))),
        "expected Io error, got: {:?}",
        result
    );
}
