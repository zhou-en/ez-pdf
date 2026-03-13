use criterion::{criterion_group, criterion_main, Criterion};
use ezpdf_core::{merge, remove, rotate, split_each};
use lopdf::{Dictionary, Document, Object, Stream};
use std::path::Path;
use tempfile::{NamedTempFile, TempDir};

fn make_pdf(page_count: u32) -> Vec<u8> {
    let mut doc = Document::with_version("1.5");
    let pages_id = doc.new_object_id();

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
                "Contents",
                Object::Array(vec![Object::Reference(content_id)]),
            ),
        ]));
        page_ids.push(Object::Reference(page_id));
    }

    doc.objects.insert(
        pages_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(page_ids)),
            ("Count", Object::Integer(page_count as i64)),
        ])),
    );
    let catalog_id = doc.add_object(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Catalog".to_vec())),
        ("Pages", Object::Reference(pages_id)),
    ]));
    doc.trailer.set("Root", Object::Reference(catalog_id));

    let mut buf = Vec::new();
    doc.save_to(&mut buf).unwrap();
    buf
}

fn write_pdf(page_count: u32) -> NamedTempFile {
    let f = NamedTempFile::new().unwrap();
    std::fs::write(f.path(), make_pdf(page_count)).unwrap();
    f
}

fn bench_merge(c: &mut Criterion) {
    let inputs: Vec<_> = (0..5).map(|_| write_pdf(10)).collect();
    let out = NamedTempFile::new().unwrap();

    c.bench_function("merge 5x10-page PDFs", |b| {
        b.iter(|| {
            let paths: Vec<&Path> = inputs.iter().map(|f| f.path()).collect();
            merge(&paths, out.path()).unwrap();
        })
    });
}

fn bench_split_each(c: &mut Criterion) {
    let input = write_pdf(50);
    let dir = TempDir::new().unwrap();

    c.bench_function("split_each 50-page PDF", |b| {
        b.iter(|| {
            split_each(input.path(), dir.path()).unwrap();
        })
    });
}

fn bench_remove(c: &mut Criterion) {
    let input = write_pdf(50);
    let out = NamedTempFile::new().unwrap();

    // Remove every other page (25 pages removed)
    let pages: String = (1u32..=50)
        .step_by(2)
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join(",");

    c.bench_function("remove half of 50-page PDF", |b| {
        b.iter(|| {
            remove(input.path(), &pages, out.path()).unwrap();
        })
    });
}

fn bench_rotate(c: &mut Criterion) {
    let input = write_pdf(50);
    let out = NamedTempFile::new().unwrap();

    c.bench_function("rotate all pages of 50-page PDF", |b| {
        b.iter(|| {
            rotate(input.path(), 90, None, out.path()).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_merge,
    bench_split_each,
    bench_remove,
    bench_rotate
);
criterion_main!(benches);
