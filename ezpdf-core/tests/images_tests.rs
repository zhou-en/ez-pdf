mod common;

use ezpdf_core::extract_images;
use std::path::Path;
use tempfile::TempDir;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Extract from a PDF with one JPEG → count > 0, at least one file created.
#[test]
fn extract_images_from_pdf_with_jpeg() {
    let dir = TempDir::new().unwrap();
    let count = extract_images(fixture("with_image.pdf").as_path(), dir.path()).unwrap();
    assert!(count > 0, "expected at least one image, got 0");

    let files: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        !files.is_empty(),
        "expected at least one file in output dir"
    );
}

/// Extract from a plain PDF with no images → count == 0, no files created.
#[test]
fn extract_images_no_images_returns_zero() {
    let dir = TempDir::new().unwrap();
    let count = extract_images(fixture("3page.pdf").as_path(), dir.path()).unwrap();
    assert_eq!(count, 0);

    let files: Vec<_> = std::fs::read_dir(dir.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        files.is_empty(),
        "expected no files in output dir for a plain PDF"
    );
}

/// Nonexistent input returns an Io error.
#[test]
fn extract_images_nonexistent_input_returns_error() {
    let dir = TempDir::new().unwrap();
    let result = extract_images(Path::new("/tmp/does_not_exist_ezpdf_img.pdf"), dir.path());
    assert!(result.is_err());
}
