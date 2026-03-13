mod common;

use ezpdf_core::{split_each, split_range};
use lopdf::Document;
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

fn page_count(path: &Path) -> u32 {
    Document::load(path)
        .expect("load output PDF")
        .get_pages()
        .len() as u32
}

// --- split_range tests ---

#[test]
fn split_range_extracts_correct_pages() {
    let input = fixture("5page.pdf");
    let out = NamedTempFile::new().unwrap();

    split_range(input.as_path(), "2-4", out.path()).unwrap();

    assert_eq!(page_count(out.path()), 3);
}

#[test]
fn split_range_single_page() {
    let input = fixture("5page.pdf");
    let out = NamedTempFile::new().unwrap();

    split_range(input.as_path(), "1", out.path()).unwrap();

    assert_eq!(page_count(out.path()), 1);
}

// --- split_each tests ---

#[test]
fn split_each_produces_one_file_per_page() {
    let input = fixture("5page.pdf");
    let dir = TempDir::new().unwrap();

    split_each(input.as_path(), dir.path()).unwrap();

    let count = std::fs::read_dir(dir.path())
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .path()
                .extension()
                .map_or(false, |e| e == "pdf")
        })
        .count();
    assert_eq!(count, 5);
}

#[test]
fn split_each_filenames_are_zero_padded() {
    let input = fixture("5page.pdf");
    let dir = TempDir::new().unwrap();

    split_each(input.as_path(), dir.path()).unwrap();

    let mut names: Vec<String> = std::fs::read_dir(dir.path())
        .unwrap()
        .map(|e| e.unwrap().file_name().to_string_lossy().to_string())
        .collect();
    names.sort();

    assert_eq!(
        names,
        vec![
            "page-1.pdf",
            "page-2.pdf",
            "page-3.pdf",
            "page-4.pdf",
            "page-5.pdf"
        ]
    );
}

#[test]
fn split_each_creates_output_dir_if_missing() {
    let input = fixture("3page.pdf");
    let dir = TempDir::new().unwrap();
    let sub = dir.path().join("new_subdir");

    // sub does not exist yet
    assert!(!sub.exists());

    split_each(input.as_path(), &sub).unwrap();

    assert!(sub.exists());
    assert_eq!(
        std::fs::read_dir(&sub)
            .unwrap()
            .filter(|e| e
                .as_ref()
                .unwrap()
                .path()
                .extension()
                .map_or(false, |e| e == "pdf"))
            .count(),
        3
    );
}

#[test]
fn split_each_each_output_file_has_one_page() {
    let input = fixture("3page.pdf");
    let dir = TempDir::new().unwrap();

    split_each(input.as_path(), dir.path()).unwrap();

    for entry in std::fs::read_dir(dir.path()).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map_or(false, |e| e == "pdf") {
            assert_eq!(page_count(&path), 1, "expected 1 page in {:?}", path);
        }
    }
}
