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

// --- merge --password ---

#[test]
fn merge_with_correct_password_succeeds() {
    let dst = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "merge",
            "--password",
            "secret",
            fixture("encrypted_pw.pdf").to_str().unwrap(),
            fixture("3page.pdf").to_str().unwrap(),
            "-o",
            dst.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn rotate_with_wrong_password_exits_nonzero_with_password_in_stderr() {
    let dst = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "rotate",
            "--password",
            "wrong",
            fixture("encrypted_pw.pdf").to_str().unwrap(),
            "90",
            "-o",
            dst.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("password"));
}

// --- verify all 5 commands accept --password ---

#[test]
fn remove_with_correct_password_succeeds() {
    let dst = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "remove",
            "--password",
            "secret",
            fixture("encrypted_pw.pdf").to_str().unwrap(),
            "1",
            "-o",
            dst.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn reorder_with_correct_password_succeeds() {
    let dst = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "reorder",
            "--password",
            "secret",
            fixture("encrypted_pw.pdf").to_str().unwrap(),
            "3,2,1",
            "-o",
            dst.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn split_with_correct_password_succeeds() {
    let dst_dir = tempfile::tempdir().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "split",
            "--password",
            "secret",
            fixture("encrypted_pw.pdf").to_str().unwrap(),
            "1-2",
            "-o",
            dst_dir.path().join("out.pdf").to_str().unwrap(),
        ])
        .assert()
        .success();
}
