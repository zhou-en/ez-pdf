use assert_cmd::Command;
use predicates::prelude::*;
use std::path::PathBuf;
use tempfile::TempDir;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("workspace root")
        .join("ezpdf-core")
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn markdown_writes_md_file_and_exits_zero() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("out.md");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["markdown", fixture("text.pdf").to_str().unwrap()])
        .args(["-o", out.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicate::str::contains("Markdown"));

    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("Quarterly Report"), "got:\n{content}");
    assert!(content.contains("The quick brown fox"), "got:\n{content}");
}

#[test]
fn markdown_pages_flag_narrows_output() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("out.md");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["markdown", fixture("text.pdf").to_str().unwrap()])
        .args(["-o", out.to_str().unwrap(), "--pages", "1"])
        .assert()
        .success();

    let content = std::fs::read_to_string(&out).unwrap();
    assert!(content.contains("Quarterly Report"));
    assert!(!content.contains("Page Three Heading"), "got:\n{content}");
}

#[test]
fn markdown_no_page_breaks_omits_separators() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("out.md");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["markdown", fixture("text.pdf").to_str().unwrap()])
        .args(["-o", out.to_str().unwrap(), "--no-page-breaks"])
        .assert()
        .success();

    let content = std::fs::read_to_string(&out).unwrap();
    assert!(
        !content.lines().any(|l| l.trim() == "---"),
        "got:\n{content}"
    );
}

#[test]
fn markdown_scanned_pdf_fails_and_names_ocr_remedy() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("out.md");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["markdown", fixture("5page.pdf").to_str().unwrap()])
        .args(["-o", out.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("ocrmypdf"));
}

#[test]
fn markdown_nonexistent_file_exits_nonzero_with_error() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("out.md");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["markdown", "/nonexistent/nope.pdf"])
        .args(["-o", out.to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Error:"));
}

#[test]
fn markdown_quiet_suppresses_stdout() {
    let dir = TempDir::new().unwrap();
    let out = dir.path().join("out.md");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["markdown", fixture("text.pdf").to_str().unwrap()])
        .args(["-o", out.to_str().unwrap(), "--quiet"])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
}
