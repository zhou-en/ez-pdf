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

/// `ezpdf bookmarks list input.pdf` exits 0.
#[test]
fn bookmarks_list_exits_zero() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["bookmarks", "list", fixture("3page.pdf").to_str().unwrap()])
        .assert()
        .success();
}

/// `ezpdf bookmarks add input.pdf --title "Ch1" --page 1 -o out.pdf` exits 0,
/// then `ezpdf bookmarks list out.pdf` stdout contains "Ch1".
#[test]
fn bookmarks_add_then_list_round_trip() {
    let out = NamedTempFile::new().unwrap();
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "bookmarks",
            "add",
            fixture("3page.pdf").to_str().unwrap(),
            "--title",
            "Ch1",
            "--page",
            "1",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["bookmarks", "list", out.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Ch1"));
}

/// Wrong input path exits non-zero.
#[test]
fn bookmarks_list_nonexistent_exits_nonzero() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["bookmarks", "list", "/tmp/does_not_exist_bm.pdf"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
