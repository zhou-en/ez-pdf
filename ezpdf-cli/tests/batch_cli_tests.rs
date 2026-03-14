use assert_cmd::Command;
use lopdf::Document;
use std::path::Path;
use tempfile::tempdir;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("ezpdf-core/tests/fixtures")
        .join(name)
}

fn page_count(path: &std::path::Path) -> u32 {
    Document::load(path)
        .expect("load output PDF")
        .get_pages()
        .len() as u32
}

fn make_batch_dir() -> (tempfile::TempDir, std::path::PathBuf, std::path::PathBuf) {
    let src = tempdir().unwrap();
    std::fs::copy(fixture("3page.pdf"), src.path().join("a.pdf")).unwrap();
    std::fs::copy(fixture("5page.pdf"), src.path().join("b.pdf")).unwrap();
    let out = tempdir().unwrap();
    let out_path = out.path().to_path_buf();
    (src, out_path, out.into_path())
}

#[test]
fn rotate_batch_processes_each_pdf_independently() {
    let src = tempdir().unwrap();
    std::fs::copy(fixture("3page.pdf"), src.path().join("a.pdf")).unwrap();
    std::fs::copy(fixture("5page.pdf"), src.path().join("b.pdf")).unwrap();
    let out = tempdir().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "rotate",
            "--batch",
            src.path().to_str().unwrap(),
            "90",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(out.path().join("a.pdf").exists());
    assert!(out.path().join("b.pdf").exists());
    assert_eq!(page_count(&out.path().join("a.pdf")), 3);
    assert_eq!(page_count(&out.path().join("b.pdf")), 5);
}

#[test]
fn remove_batch_processes_each_pdf_independently() {
    let src = tempdir().unwrap();
    std::fs::copy(fixture("3page.pdf"), src.path().join("a.pdf")).unwrap();
    std::fs::copy(fixture("5page.pdf"), src.path().join("b.pdf")).unwrap();
    let out = tempdir().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "remove",
            "--batch",
            src.path().to_str().unwrap(),
            "1",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(page_count(&out.path().join("a.pdf")), 2);
    assert_eq!(page_count(&out.path().join("b.pdf")), 4);
}

#[test]
fn merge_batch_merges_all_pdfs_in_dir_into_one_file() {
    let src = tempdir().unwrap();
    std::fs::copy(fixture("3page.pdf"), src.path().join("a.pdf")).unwrap();
    std::fs::copy(fixture("5page.pdf"), src.path().join("b.pdf")).unwrap();
    let out = tempdir().unwrap();
    let out_file = out.path().join("merged.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "merge",
            "--batch",
            src.path().to_str().unwrap(),
            "-o",
            out_file.to_str().unwrap(),
        ])
        .assert()
        .success();

    assert!(out_file.exists());
    assert_eq!(page_count(&out_file), 8);
}

#[test]
fn rotate_batch_nonexistent_dir_exits_with_error() {
    let out = tempdir().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "rotate",
            "--batch",
            "/tmp/does_not_exist_ezpdf_batch/",
            "90",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
