use assert_cmd::Command;
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("ezpdf-core/tests/fixtures")
        .join(name)
}

#[test]
fn merge_two_pdfs_exits_zero_and_prints_merged() {
    let out = NamedTempFile::new().unwrap();
    let a = fixture("3page.pdf");
    let b = fixture("5page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["merge", a.to_str().unwrap(), b.to_str().unwrap(), "-o", out.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Merged"));
}

#[test]
fn merge_nonexistent_file_exits_nonzero_with_error() {
    let out = NamedTempFile::new().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["merge", "/tmp/does_not_exist_ezpdf.pdf", "-o", out.path().to_str().unwrap()])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
