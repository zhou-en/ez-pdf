use assert_cmd::Command;
use lopdf::Document;
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("ezpdf-core/tests/fixtures")
        .join(name)
}

fn page_count(path: &Path) -> u32 {
    Document::load(path)
        .expect("load output PDF")
        .get_pages()
        .len() as u32
}

#[test]
fn split_range_mode_exits_zero_and_prints_split() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("5page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "split",
            input.to_str().unwrap(),
            "1-3",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Split"));

    assert_eq!(page_count(out.path()), 3);
}

#[test]
fn split_each_mode_exits_zero_and_creates_files() {
    let input = fixture("3page.pdf");
    let dir = TempDir::new().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "split",
            input.to_str().unwrap(),
            "--each",
            "-o",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Split"));

    let pdf_count = std::fs::read_dir(dir.path())
        .unwrap()
        .filter(|e| {
            e.as_ref()
                .unwrap()
                .path()
                .extension()
                .map_or(false, |e| e == "pdf")
        })
        .count();
    assert_eq!(pdf_count, 3);
}
