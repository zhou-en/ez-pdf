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

/// `ezpdf optimize input.pdf -o out.pdf` exits 0, stdout contains "Optimized".
#[test]
fn optimize_exits_zero_and_prints_optimized() {
    let out = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "optimize",
            fixture("bloated.pdf").to_str().unwrap(),
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Optimized"));
}

/// `ezpdf optimize input.pdf --linearize -o out.pdf` exits 0.
#[test]
fn optimize_linearize_flag_exits_zero() {
    let out = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "optimize",
            fixture("3page.pdf").to_str().unwrap(),
            "--linearize",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

/// Nonexistent input exits non-zero.
#[test]
fn optimize_nonexistent_input_exits_nonzero() {
    let out = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "optimize",
            "/tmp/does_not_exist_ezpdf_opt.pdf",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
