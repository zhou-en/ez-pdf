mod common;

use ezpdf_core::{batch::collect_pdf_inputs, error::EzPdfError};
use std::fs;
use std::path::Path;
use tempfile::tempdir;

#[test]
fn collect_returns_pdf_files_sorted_alphabetically() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("c.pdf"), b"").unwrap();
    fs::write(dir.path().join("a.pdf"), b"").unwrap();
    fs::write(dir.path().join("b.pdf"), b"").unwrap();

    let result = collect_pdf_inputs(dir.path()).unwrap();

    let names: Vec<&str> = result
        .iter()
        .map(|p| p.file_name().unwrap().to_str().unwrap())
        .collect();
    assert_eq!(names, vec!["a.pdf", "b.pdf", "c.pdf"]);
}

#[test]
fn collect_filters_out_non_pdf_files() {
    let dir = tempdir().unwrap();
    fs::write(dir.path().join("doc.pdf"), b"").unwrap();
    fs::write(dir.path().join("notes.txt"), b"").unwrap();
    fs::write(dir.path().join("image.png"), b"").unwrap();

    let result = collect_pdf_inputs(dir.path()).unwrap();

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].file_name().unwrap(), "doc.pdf");
}

#[test]
fn collect_nonexistent_dir_returns_io_error() {
    let path = Path::new("/tmp/does_not_exist_ezpdf_batch_dir");
    let result = collect_pdf_inputs(path);
    assert!(
        matches!(result, Err(EzPdfError::Io(_))),
        "expected Io error, got: {:?}",
        result
    );
}

#[test]
fn collect_empty_dir_returns_empty_vec() {
    let dir = tempdir().unwrap();
    let result = collect_pdf_inputs(dir.path()).unwrap();
    assert!(result.is_empty());
}
