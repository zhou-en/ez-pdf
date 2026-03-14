use assert_cmd::Command;
use std::path::Path;
use tempfile::TempDir;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("ezpdf-core/tests/fixtures")
        .join(name)
}

/// `ezpdf images with_image.pdf -o dir/` exits 0 and prints "Extracted".
#[test]
fn images_with_jpeg_exits_zero_and_prints_extracted() {
    let dir = TempDir::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "images",
            fixture("with_image.pdf").to_str().unwrap(),
            "-o",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Extracted"));
}

/// `ezpdf images 3page.pdf -o dir/` exits 0 and stdout contains "0 image".
#[test]
fn images_no_images_exits_zero_and_prints_zero() {
    let dir = TempDir::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "images",
            fixture("3page.pdf").to_str().unwrap(),
            "-o",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("0 image"));
}

/// Nonexistent input exits non-zero.
#[test]
fn images_nonexistent_input_exits_nonzero() {
    let dir = TempDir::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "images",
            "/tmp/does_not_exist_ezpdf_img.pdf",
            "-o",
            dir.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
