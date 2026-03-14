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
fn watermark_basic_succeeds() {
    let out = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "watermark",
            fixture("3page.pdf").to_str().unwrap(),
            "DRAFT",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn watermark_with_pages_flag_succeeds() {
    let out = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "watermark",
            fixture("3page.pdf").to_str().unwrap(),
            "DRAFT",
            "--pages",
            "1,3",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success();
}

#[test]
fn watermark_nonexistent_input_exits_nonzero() {
    let out = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "watermark",
            "/tmp/does_not_exist_ezpdf_wm.pdf",
            "DRAFT",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
