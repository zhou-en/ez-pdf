use assert_cmd::Command;
use lopdf::Document;
use std::path::Path;
use tempfile::NamedTempFile;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("ezpdf-core/tests/fixtures")
        .join(name)
}

fn get_page_rotation(path: &Path, page_num: u32) -> i64 {
    let doc = Document::load(path).expect("load PDF");
    let pages = doc.get_pages();
    let page_id = *pages.get(&page_num).expect("page");
    let page = doc.get_object(page_id).expect("object");
    let dict = page.as_dict().expect("dict");
    dict.get(b"Rotate").and_then(|r| r.as_i64()).unwrap_or(0)
}

#[test]
fn rotate_all_pages_exits_zero_and_prints_rotated() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "rotate",
            input.to_str().unwrap(),
            "90",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicates::str::contains("Rotated"));

    assert_eq!(get_page_rotation(out.path(), 1), 90);
}

#[test]
fn rotate_specific_pages_via_flag() {
    let out = NamedTempFile::new().unwrap();
    let input = fixture("3page.pdf");

    Command::cargo_bin("ezpdf")
        .unwrap()
        .args([
            "rotate",
            input.to_str().unwrap(),
            "90",
            "--pages",
            "1,3",
            "-o",
            out.path().to_str().unwrap(),
        ])
        .assert()
        .success();

    assert_eq!(get_page_rotation(out.path(), 1), 90);
    assert_eq!(get_page_rotation(out.path(), 2), 0);
    assert_eq!(get_page_rotation(out.path(), 3), 90);
}
