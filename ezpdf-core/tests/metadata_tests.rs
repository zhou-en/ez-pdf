mod common;

use ezpdf_core::{
    get_metadata, set_metadata,
    metadata::{PdfMetadata, MetadataUpdate},
};
use std::path::Path;
use tempfile::NamedTempFile;

fn write_temp_pdf(data: &[u8]) -> NamedTempFile {
    let f = NamedTempFile::new().unwrap();
    std::fs::write(f.path(), data).unwrap();
    f
}

#[test]
fn get_metadata_on_fixture_returns_no_error() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/3page.pdf");
    let result = get_metadata(&path);
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
}

#[test]
fn set_then_get_title_matches() {
    let src = write_temp_pdf(&common::create_test_pdf(1, "title-test"));
    let dst = NamedTempFile::new().unwrap();

    set_metadata(
        src.path(),
        MetadataUpdate { title: Some("Hello World".to_string()), ..Default::default() },
        dst.path(),
    )
    .unwrap();

    let meta = get_metadata(dst.path()).unwrap();
    assert_eq!(meta.title.as_deref(), Some("Hello World"));
}

#[test]
fn set_multiple_fields_all_updated() {
    let src = write_temp_pdf(&common::create_test_pdf(1, "multi-field-test"));
    let dst = NamedTempFile::new().unwrap();

    set_metadata(
        src.path(),
        MetadataUpdate {
            title: Some("My Title".to_string()),
            author: Some("Jane Doe".to_string()),
            subject: Some("Testing".to_string()),
            ..Default::default()
        },
        dst.path(),
    )
    .unwrap();

    let meta = get_metadata(dst.path()).unwrap();
    assert_eq!(meta.title.as_deref(), Some("My Title"));
    assert_eq!(meta.author.as_deref(), Some("Jane Doe"));
    assert_eq!(meta.subject.as_deref(), Some("Testing"));
}

#[test]
fn clear_all_wipes_all_fields() {
    // First set some fields
    let src = write_temp_pdf(&common::create_test_pdf(1, "clear-all-test"));
    let step1 = NamedTempFile::new().unwrap();

    set_metadata(
        src.path(),
        MetadataUpdate {
            title: Some("Will Be Cleared".to_string()),
            author: Some("Also Cleared".to_string()),
            ..Default::default()
        },
        step1.path(),
    )
    .unwrap();

    // Then clear all
    let step2 = NamedTempFile::new().unwrap();
    set_metadata(
        step1.path(),
        MetadataUpdate { clear_all: true, ..Default::default() },
        step2.path(),
    )
    .unwrap();

    let meta = get_metadata(step2.path()).unwrap();
    assert!(meta.title.is_none(), "title should be cleared, got: {:?}", meta.title);
    assert!(meta.author.is_none(), "author should be cleared, got: {:?}", meta.author);
}

// Shape check: PdfMetadata must have all required fields
#[allow(dead_code)]
fn _shape_check(m: PdfMetadata) {
    let _: Option<String> = m.title;
    let _: Option<String> = m.author;
    let _: Option<String> = m.subject;
    let _: Option<String> = m.keywords;
    let _: Option<String> = m.creator;
    let _: Option<String> = m.producer;
}

// Shape check: MetadataUpdate must have all required fields
#[allow(dead_code)]
fn _update_shape_check(u: MetadataUpdate) {
    let _: Option<String> = u.title;
    let _: Option<String> = u.author;
    let _: Option<String> = u.subject;
    let _: Option<String> = u.keywords;
    let _: Option<String> = u.creator;
    let _: Option<String> = u.producer;
    let _: bool = u.clear_all;
}
