mod common;

use ezpdf_core::{optimize, OptimizeStats};
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Optimizing a bloated PDF removes at least one unreferenced object.
#[test]
fn optimize_bloated_removes_objects() {
    let out = NamedTempFile::new().unwrap();
    let stats: OptimizeStats = optimize(fixture("bloated.pdf").as_path(), out.path()).unwrap();
    assert!(
        stats.objects_removed > 0,
        "expected unreferenced objects removed, got 0"
    );
}

/// Optimizing a plain PDF produces a valid PDF with the same page count.
#[test]
fn optimize_plain_pdf_is_valid() {
    let out = NamedTempFile::new().unwrap();
    let _stats = optimize(fixture("3page.pdf").as_path(), out.path()).unwrap();

    let doc = lopdf::Document::load(out.path()).unwrap();
    assert_eq!(doc.get_pages().len(), 3, "page count should be preserved");
}

/// Optimizing preserves page count on the bloated fixture too.
#[test]
fn optimize_bloated_preserves_page_count() {
    let out = NamedTempFile::new().unwrap();
    let _stats = optimize(fixture("bloated.pdf").as_path(), out.path()).unwrap();

    let original = lopdf::Document::load(fixture("bloated.pdf").as_path()).unwrap();
    let optimized = lopdf::Document::load(out.path()).unwrap();
    assert_eq!(
        original.get_pages().len(),
        optimized.get_pages().len(),
        "page count should be preserved"
    );
}
