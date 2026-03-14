use assert_cmd::Command;
use std::path::Path;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("ezpdf-core/tests/fixtures")
        .join(name)
}

#[test]
fn info_prints_page_count() {
    let f = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["info", f.to_str().unwrap()])
        .assert()
        .success()
        .stdout(predicates::str::contains("Pages: 3"));
}

#[test]
fn info_json_flag_outputs_parseable_json_with_page_count() {
    let f = fixture("3page.pdf");

    let output = Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["info", "--json", f.to_str().unwrap()])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let text = String::from_utf8(output).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&text).expect("stdout should be valid JSON");
    assert_eq!(parsed["page_count"], 3);
}

#[test]
fn info_nonexistent_file_exits_nonzero_with_error() {
    Command::cargo_bin("ezpdf")
        .unwrap()
        .args(["info", "/tmp/does_not_exist_ezpdf_info.pdf"])
        .assert()
        .failure()
        .stderr(predicates::str::contains("Error:"));
}
