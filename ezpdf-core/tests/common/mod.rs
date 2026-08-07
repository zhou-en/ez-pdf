use lopdf::{Dictionary, Document, Object, Stream};

/// Creates a minimal valid PDF with `page_count` blank pages.
/// The `label` is embedded as the document title for debugging.
pub fn create_test_pdf(page_count: u32, label: &str) -> Vec<u8> {
    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();
    let font_id = doc.add_object(Stream::new(
        Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Font".to_vec())),
            ("Subtype", Object::Name(b"Type1".to_vec())),
            ("BaseFont", Object::Name(b"Helvetica".to_vec())),
        ]),
        vec![],
    ));

    let mut page_ids = Vec::new();
    for _ in 0..page_count {
        let content_id = doc.add_object(Stream::new(Dictionary::new(), vec![]));
        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Page".to_vec())),
            ("Parent", Object::Reference(pages_id)),
            (
                "MediaBox",
                Object::Array(vec![
                    Object::Integer(0),
                    Object::Integer(0),
                    Object::Integer(612),
                    Object::Integer(792),
                ]),
            ),
            (
                "Resources",
                Object::Dictionary(Dictionary::from_iter(vec![(
                    "Font",
                    Object::Dictionary(Dictionary::from_iter(vec![(
                        "F1",
                        Object::Reference(font_id),
                    )])),
                )])),
            ),
            (
                "Contents",
                Object::Array(vec![Object::Reference(content_id)]),
            ),
        ]));
        page_ids.push(Object::Reference(page_id));
    }

    let pages_dict = Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Pages".to_vec())),
        ("Kids", Object::Array(page_ids)),
        ("Count", Object::Integer(page_count as i64)),
    ]);
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
        (
            "Info",
            Object::Dictionary(Dictionary::from_iter(vec![(
                "Title",
                Object::string_literal(label),
            )])),
        ),
    ]));
    doc.trailer.set("Root", Object::Reference(catalog_id));

    let mut buf = Vec::new();
    doc.save_to(&mut buf).expect("failed to serialize test PDF");
    buf
}

/// Creates a PDF whose pages carry real text content streams.
///
/// Each entry in `pages` is a `(heading, body)` pair. The heading is drawn at
/// 24pt and the body at 11pt so that font-size-based heading detection has
/// something to work with.
pub fn create_test_pdf_with_text(pages: &[(&str, &str)]) -> Vec<u8> {
    use lopdf::content::{Content, Operation};

    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();
    // A real font dictionary (not a stream) with an explicit encoding — text
    // extractors need this to map glyph codes back to characters.
    let font_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
        ("Encoding", Object::Name(b"WinAnsiEncoding".to_vec())),
    ]));

    let mut page_ids = Vec::new();
    for (heading, body) in pages {
        let content = Content {
            operations: vec![
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 24.into()]),
                Operation::new("Td", vec![72.into(), 700.into()]),
                Operation::new("Tj", vec![Object::string_literal(*heading)]),
                Operation::new("ET", vec![]),
                Operation::new("BT", vec![]),
                Operation::new("Tf", vec!["F1".into(), 11.into()]),
                Operation::new("Td", vec![72.into(), 650.into()]),
                Operation::new("Tj", vec![Object::string_literal(*body)]),
                Operation::new("ET", vec![]),
            ],
        };
        let content_id = doc.add_object(Stream::new(
            Dictionary::new(),
            content.encode().expect("failed to encode content stream"),
        ));

        let page_id = doc.add_object(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Page".to_vec())),
            ("Parent", Object::Reference(pages_id)),
            (
                "MediaBox",
                Object::Array(vec![
                    Object::Integer(0),
                    Object::Integer(0),
                    Object::Integer(612),
                    Object::Integer(792),
                ]),
            ),
            (
                "Resources",
                Object::Dictionary(Dictionary::from_iter(vec![(
                    "Font",
                    Object::Dictionary(Dictionary::from_iter(vec![(
                        "F1",
                        Object::Reference(font_id),
                    )])),
                )])),
            ),
            (
                "Contents",
                Object::Array(vec![Object::Reference(content_id)]),
            ),
        ]));
        page_ids.push(Object::Reference(page_id));
    }

    let pages_dict = Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Pages".to_vec())),
        ("Kids", Object::Array(page_ids)),
        ("Count", Object::Integer(pages.len() as i64)),
    ]);
    doc.objects.insert(pages_id, Object::Dictionary(pages_dict));

    let catalog_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    doc.trailer.set("Root", Object::Reference(catalog_id));

    let mut buf = Vec::new();
    doc.save_to(&mut buf).expect("failed to serialize text PDF");
    buf
}

/// The page content backing the committed `text.pdf` fixture.
#[allow(dead_code)]
pub const TEXT_FIXTURE_PAGES: [(&str, &str); 3] = [
    (
        "Quarterly Report",
        "The quick brown fox jumps over the lazy dog.",
    ),
    ("Page Two Heading", "Revenue grew across every region."),
    ("Page Three Heading", "Closing notes and acknowledgements."),
];

/// Creates a minimal PDF that lopdf treats as encrypted (has /Encrypt in trailer).
pub fn create_encrypted_pdf() -> Vec<u8> {
    let mut doc = Document::with_version("1.5");

    let pages_id = doc.new_object_id();
    let page_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Page".to_vec())),
        ("Parent", Object::Reference(pages_id)),
        (
            "MediaBox",
            Object::Array(vec![
                Object::Integer(0),
                Object::Integer(0),
                Object::Integer(612),
                Object::Integer(792),
            ]),
        ),
    ]));

    doc.objects.insert(
        pages_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(vec![Object::Reference(page_id)])),
            ("Count", Object::Integer(1)),
        ])),
    );

    let catalog_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    doc.trailer.set("Root", Object::Reference(catalog_id));

    // Add a stub /Encrypt dict — sufficient for is_encrypted() to return true
    let encrypt_id = doc.add_object(Dictionary::from_iter(vec![
        ("Filter", Object::Name(b"Standard".to_vec())),
        ("V", Object::Integer(2)),
        ("R", Object::Integer(3)),
        ("Length", Object::Integer(128)),
    ]));
    doc.trailer.set("Encrypt", Object::Reference(encrypt_id));

    let mut buf = Vec::new();
    doc.save_to(&mut buf)
        .expect("failed to serialize encrypted test PDF");
    buf
}

/// Write fixture PDFs to disk (call this from a test marked `#[ignore]`).
#[allow(dead_code)]
pub fn write_fixtures() {
    use std::path::Path;

    let fixtures = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures");

    let pdf_3 = create_test_pdf(3, "3-page fixture");
    std::fs::write(fixtures.join("3page.pdf"), pdf_3).expect("write 3page.pdf");

    let pdf_5 = create_test_pdf(5, "5-page fixture");
    std::fs::write(fixtures.join("5page.pdf"), pdf_5).expect("write 5page.pdf");

    let pdf_enc = create_encrypted_pdf();
    std::fs::write(fixtures.join("encrypted.pdf"), pdf_enc).expect("write encrypted.pdf");

    let pdf_text = create_test_pdf_with_text(&TEXT_FIXTURE_PAGES);
    std::fs::write(fixtures.join("text.pdf"), pdf_text).expect("write text.pdf");
}
