use assert_cmd::Command;
use lopdf::Document;
use std::path::Path;
use tempfile::NamedTempFile;

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
fn remove_page_exits_zero_and_prints_removed() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("5page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "remove",
            input.to_str().unwrap(),
            "3",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Removed"));

    assert_eq!(page_count(out.path()), 4);
}

#[test]
fn remove_nonexistent_file_exits_nonzero_with_error() {
    let out = NamedTempFile::new().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "remove",
            "/tmp/does_not_exist_ezpdf.pdf",
            "1",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
