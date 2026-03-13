use std::path::Path;

use lopdf::{Dictionary, Document, Object};

use crate::error::EzPdfError;

pub fn merge(inputs: &[&Path], output: &Path) -> Result<(), EzPdfError> {
    let docs: Vec<Document> = inputs
        .iter()
        .map(|p| load_doc(p))
        .collect::<Result<_, _>>()?;

    let mut result = build_merged(docs)?;

    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    result
        .save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

fn build_merged(mut docs: Vec<Document>) -> Result<Document, EzPdfError> {
    let mut result = Document::with_version("1.5");

    // Renumber each source doc so object IDs don't collide
    let mut next_id: u32 = 1;
    for doc in docs.iter_mut() {
        doc.renumber_objects_with(next_id);
        next_id = doc.max_id + 1;
    }

    // Collect page IDs (in document order) after renumbering
    let all_page_ids: Vec<_> = docs
        .iter()
        .flat_map(|doc| {
            let mut pages: Vec<_> = doc.get_pages().into_iter().collect();
            pages.sort_by_key(|(n, _)| *n);
            pages.into_iter().map(|(_, id)| id)
        })
        .collect();

    // Move all objects from source docs into result
    let mut max_id = 0u32;
    for doc in docs {
        max_id = max_id.max(doc.max_id);
        result.objects.extend(doc.objects);
    }
    result.max_id = max_id;

    // Build a fresh /Pages root containing all pages
    let page_count = all_page_ids.len() as i64;
    let kids: Vec<Object> = all_page_ids
        .iter()
        .map(|&id| Object::Reference(id))
        .collect();

    let pages_root_id = result.new_object_id();
    result.objects.insert(
        pages_root_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(kids)),
            ("Count", Object::Integer(page_count)),
        ])),
    );

    // Update every page's /Parent to point to the new root
    for &page_id in &all_page_ids {
        if let Ok(page) = result.get_object_mut(page_id) {
            if let Ok(dict) = page.as_dict_mut() {
                dict.set("Parent", Object::Reference(pages_root_id));
            }
        }
    }

    // Build a fresh /Catalog
    let catalog_id = result.new_object_id();
    result.objects.insert(
        catalog_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", Object::Reference(pages_root_id)),
        ])),
    );
    result.trailer.set("Root", Object::Reference(catalog_id));

    Ok(result)
}

pub(crate) fn load_doc(path: &Path) -> Result<Document, EzPdfError> {
    let doc = Document::load(path).map_err(|e| match e {
        lopdf::Error::IO(io_err) => EzPdfError::Io(std::io::Error::new(
            io_err.kind(),
            format!("{}: {io_err}", path.display()),
        )),
        lopdf::Error::Decryption(_) => EzPdfError::EncryptedPdf,
        other => EzPdfError::Io(std::io::Error::other(format!(
            "{}: {other}",
            path.display()
        ))),
    })?;
    if doc.is_encrypted() {
        return Err(EzPdfError::EncryptedPdf);
    }
    Ok(doc)
}
