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
fn meta_get_exits_zero_and_prints_fields() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["meta", "get", fixture("3page.pdf").to_str().unwrap()])
        .assert()
        .success();
}

#[test]
fn meta_get_json_is_valid_json() {
    let output = Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "meta",
            "get",
            "--json",
            fixture("3page.pdf").to_str().unwrap(),
        ])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();
    serde_json::from_str::<serde_json::Value>(&text).expect("stdout should be valid JSON");
}

#[test]
fn meta_set_title_and_get_shows_title() {
    let dst = NamedTempFile::new().unwrap();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "meta",
            "set",
            fixture("3page.pdf").to_str().unwrap(),
            "--title",
            "Round-Trip Title",
            "-o",
            dst.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["meta", "get", dst.path().to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Round-Trip Title"));
}

#[test]
fn meta_get_nonexistent_file_exits_nonzero() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["meta", "get", "/tmp/does_not_exist_ezpdf_meta.pdf"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
