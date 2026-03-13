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

fn page_count(path: &std::path::Path) -> u32 {
    Document::load(path)
        .expect("load output PDF")
        .get_pages()
        .len() as u32
}

#[test]
fn reorder_exits_zero_and_prints_reordered() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "reorder",
            input.to_str().unwrap(),
            "3,1,2",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Reordered"));

    assert_eq!(page_count(out.path()), 3);
}

#[test]
fn reorder_duplicate_page_exits_nonzero() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "reorder",
            input.to_str().unwrap(),
            "1,1,2",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn reorder_missing_page_exits_nonzero() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "reorder",
            input.to_str().unwrap(),
            "1,2",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn reorder_out_of_range_exits_nonzero() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "reorder",
            input.to_str().unwrap(),
            "1,2,9",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure();
}

#[test]
fn reorder_quiet_flag_suppresses_output() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "reorder",
            input.to_str().unwrap(),
            "2,3,1",
            "-o",
            out.path().to_str().unwrap(),
            "-q",
        ])
        .assert()
        .success()
        .stdout("");
}
