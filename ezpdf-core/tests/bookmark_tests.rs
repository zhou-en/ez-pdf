mod common;

use ezpdf_core::{add_bookmark, list_bookmarks};
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// A plain PDF with no /Outlines produces an empty bookmark list.
#[test]
fn list_bookmarks_no_outline_returns_empty() {
    let result = list_bookmarks(fixture("3page.pdf").as_path()).unwrap();
    assert!(
        result.is_empty(),
        "expected no bookmarks on a plain PDF, got {:?}",
        result
    );
}

/// After adding a bookmark, listing returns the entry with correct title and page.
#[test]
fn add_then_list_shows_entry() {
    let out = NamedTempFile::new().unwrap();
    add_bookmark(fixture("3page.pdf").as_path(), "Chapter 1", 2, out.path()).unwrap();

    let bookmarks = list_bookmarks(out.path()).unwrap();
    assert_eq!(bookmarks.len(), 1);
    assert_eq!(bookmarks[0].title, "Chapter 1");
    assert_eq!(bookmarks[0].page, 2);
}

/// Adding two bookmarks produces both entries in insertion order.
#[test]
fn add_two_bookmarks_both_present_in_order() {
    let tmp1 = NamedTempFile::new().unwrap();
    add_bookmark(fixture("3page.pdf").as_path(), "Intro", 1, tmp1.path()).unwrap();

    let tmp2 = NamedTempFile::new().unwrap();
    add_bookmark(tmp1.path(), "Conclusion", 3, tmp2.path()).unwrap();

    let bookmarks = list_bookmarks(tmp2.path()).unwrap();
    assert_eq!(bookmarks.len(), 2);
    assert_eq!(bookmarks[0].title, "Intro");
    assert_eq!(bookmarks[0].page, 1);
    assert_eq!(bookmarks[1].title, "Conclusion");
    assert_eq!(bookmarks[1].page, 3);
}

/// `Bookmark` has a `level` field (0 = top-level).
#[test]
fn bookmark_level_is_zero_for_top_level() {
    let out = NamedTempFile::new().unwrap();
    add_bookmark(fixture("3page.pdf").as_path(), "Top", 1, out.path()).unwrap();
    let bookmarks = list_bookmarks(out.path()).unwrap();
    assert_eq!(bookmarks[0].level, 0);
}
