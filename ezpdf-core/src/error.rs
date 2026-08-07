use thiserror::Error;

#[derive(Debug, Error)]
pub enum EzPdfError {
    #[error("page {page} is out of range (document has {total} pages)")]
    PageOutOfRange { page: u32, total: u32 },

    #[error("invalid page range syntax '{input}': {hint}")]
    InvalidSyntax { input: String, hint: String },

    #[error(
        "PDF is password-protected; decrypt it first with: qpdf --decrypt input.pdf output.pdf"
    )]
    EncryptedPdf,

    #[error("wrong PDF password; check the password and try again")]
    WrongPassword,

    #[error(
        "no extractable text layer on page(s) {pages:?}; this looks like a scanned PDF. \
         Run it through an OCR tool first with: ocrmypdf input.pdf output.pdf"
    )]
    NoTextLayer { pages: Vec<u32> },

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("PDF error: {0}")]
    Pdf(String),
}
