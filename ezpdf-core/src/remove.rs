use std::path::Path;

use lopdf::{Dictionary, Document, Object};

use crate::error::EzPdfError;
use crate::merge::{load_doc, load_doc_mem, save_to_vec};
use crate::page_range;

pub fn remove(input: &Path, pages: &str, output: &Path) -> Result<(), EzPdfError> {
    let mut result = build_removed(load_doc(input)?, pages)?;
    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    result
        .save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

/// [`remove`] over an in-memory PDF.
pub fn remove_bytes(input: &[u8], pages: &str) -> Result<Vec<u8>, EzPdfError> {
    save_to_vec(build_removed(load_doc_mem(input, None)?, pages)?)
}

/// Drops `pages` from `doc`, refusing to leave it empty.
fn build_removed(doc: Document, pages: &str) -> Result<Document, EzPdfError> {
    let page_count = doc.get_pages().len() as u32;
    let pages_to_remove = page_range::parse(pages, page_count)?;

    let keep: Vec<u32> = (1..=page_count)
        .filter(|p| !pages_to_remove.contains(p))
        .collect();

    if keep.is_empty() {
        return Err(EzPdfError::InvalidSyntax {
            input: pages.to_string(),
            hint: format!("cannot remove all {page_count} pages from a document"),
        });
    }

    build_kept(doc, &keep)
}

pub(crate) fn build_kept(mut doc: Document, keep: &[u32]) -> Result<Document, EzPdfError> {
    let all_pages = doc.get_pages();

    // Get page IDs to keep in order
    let kept_ids: Vec<_> = keep
        .iter()
        .filter_map(|n| all_pages.get(n).copied())
        .collect();

    // Build fresh pages root
    let pages_root_id = doc.new_object_id();
    let page_count = kept_ids.len() as i64;
    let kids: Vec<Object> = kept_ids.iter().map(|&id| Object::Reference(id)).collect();

    doc.objects.insert(
        pages_root_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(kids)),
            ("Count", Object::Integer(page_count)),
        ])),
    );

    // Update each kept page's /Parent
    for &page_id in &kept_ids {
        if let Ok(page) = doc.get_object_mut(page_id) {
            if let Ok(dict) = page.as_dict_mut() {
                dict.set("Parent", Object::Reference(pages_root_id));
            }
        }
    }

    // Replace catalog's Pages reference
    let catalog_id = doc.new_object_id();
    doc.objects.insert(
        catalog_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", Object::Reference(pages_root_id)),
        ])),
    );
    doc.trailer.set("Root", Object::Reference(catalog_id));

    Ok(doc)
}
