mod common;

use ezpdf_core::{
    watermark,
    watermark_mod::WatermarkOptions,
};
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn default_opts() -> WatermarkOptions {
    WatermarkOptions {
        opacity: 0.3,
        color_rgb: (0.5, 0.5, 0.5),
        font_size: 48.0,
        pages: None,
    }
}

/// Watermarking preserves page count
#[test]
fn watermark_preserves_page_count() {
    let out = NamedTempFile::new().unwrap();
    watermark(fixture("3page.pdf").as_path(), "CONFIDENTIAL", default_opts(), out.path()).unwrap();

    let doc = lopdf::Document::load(out.path()).unwrap();
    assert_eq!(doc.get_pages().len(), 3);
}

/// Output PDF binary contains the watermark text
#[test]
fn watermark_text_appears_in_output_bytes() {
    let out = NamedTempFile::new().unwrap();
    watermark(fixture("3page.pdf").as_path(), "DRAFT", default_opts(), out.path()).unwrap();

    let bytes = std::fs::read(out.path()).unwrap();
    assert!(
        bytes.windows(5).any(|w| w == b"DRAFT"),
        "expected 'DRAFT' to appear somewhere in the output PDF bytes"
    );
}
