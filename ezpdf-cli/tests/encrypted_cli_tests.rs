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
fn merge_encrypted_input_exits_nonzero_with_hint() {
    let out = NamedTempFile::new().unwrap();
    let enc = fixture("encrypted.pdf");
    let plain = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "merge",
            enc.to_str().unwrap(),
            plain.to_str().unwrap(),
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("password-protected"));
}

#[test]
fn remove_encrypted_input_exits_nonzero_with_hint() {
    let out = NamedTempFile::new().unwrap();
    let enc = fixture("encrypted.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "remove",
            enc.to_str().unwrap(),
            "1",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("password-protected"));
}

#[test]
fn split_encrypted_input_exits_nonzero_with_hint() {
    let out = NamedTempFile::new().unwrap();
    let enc = fixture("encrypted.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "split",
            enc.to_str().unwrap(),
            "1",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("password-protected"));
}

#[test]
fn reorder_encrypted_input_exits_nonzero_with_hint() {
    let out = NamedTempFile::new().unwrap();
    let enc = fixture("encrypted.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "reorder",
            enc.to_str().unwrap(),
            "1",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("password-protected"));
}

#[test]
fn rotate_encrypted_input_exits_nonzero_with_hint() {
    let out = NamedTempFile::new().unwrap();
    let enc = fixture("encrypted.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "rotate",
            enc.to_str().unwrap(),
            "90",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("password-protected"));
}

#[test]
fn merge_nonexistent_file_exits_nonzero_with_path_in_error() {
    let out = NamedTempFile::new().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "merge",
            "/nonexistent/path/file.pdf",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("nonexistent"));
}
