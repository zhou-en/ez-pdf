mod common;

/// Run once to generate static fixture PDFs.
/// These are committed to the repo so normal tests don't need to regenerate.
#[test]
#[ignore]
fn generate_fixture_pdfs() {
    common::write_fixtures();
}
