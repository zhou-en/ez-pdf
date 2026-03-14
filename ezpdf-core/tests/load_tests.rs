use ezpdf_core::{error::EzPdfError, load_doc_with_password};
use std::path::Path;

fn fixture(name: &str) -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

/// Correct password → document loads and has expected page count
#[test]
fn correct_password_loads_document() {
    let path = fixture("encrypted_pw.pdf");
    let result = load_doc_with_password(&path, Some("secret"));
    assert!(result.is_ok(), "expected Ok, got: {:?}", result);
    let doc = result.unwrap();
    assert_eq!(doc.get_pages().len(), 3, "expected 3 pages");
}

/// Wrong password → WrongPassword error
#[test]
fn wrong_password_returns_wrong_password_error() {
    let path = fixture("encrypted_pw.pdf");
    let result = load_doc_with_password(&path, Some("hunter2"));
    assert!(
        matches!(result, Err(EzPdfError::WrongPassword)),
        "expected WrongPassword, got: {:?}",
        result
    );
}

/// No password on encrypted PDF → EncryptedPdf error (unchanged behaviour)
#[test]
fn no_password_on_encrypted_returns_encrypted_pdf_error() {
    let path = fixture("encrypted_pw.pdf");
    let result = load_doc_with_password(&path, None);
    assert!(
        matches!(result, Err(EzPdfError::EncryptedPdf)),
        "expected EncryptedPdf, got: {:?}",
        result
    );
}
