use std::path::Path;

use lopdf::{Dictionary, Object, ObjectId};

use crate::error::EzPdfError;
use crate::merge::load_doc;

/// A single entry in the PDF outline (bookmark) tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bookmark {
    pub title: String,
    pub page: u32,
    pub level: u32,
}

/// Return all top-level bookmarks from the PDF's /Outlines tree.
pub fn list_bookmarks(input: &Path) -> Result<Vec<Bookmark>, EzPdfError> {
    let doc = load_doc(input)?;

    let outlines_id = match outline_root_id(&doc) {
        Some(id) => id,
        None => return Ok(vec![]),
    };

    let first_id: Option<ObjectId> = {
        let outlines_obj = doc
            .get_object(outlines_id)
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
        let outlines_dict = outlines_obj
            .as_dict()
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
        dict_ref(outlines_dict, b"First")
    };

    let pages_map = doc.get_pages(); // BTreeMap<u32, ObjectId>

    let mut bookmarks = Vec::new();
    let mut cursor = first_id;
    while let Some(item_id) = cursor {
        let item_obj = doc
            .get_object(item_id)
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
        let item_dict = item_obj
            .as_dict()
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;

        let title = read_pdf_string(item_dict, b"Title")?;
        let page = dest_page_number(item_dict, &pages_map, &doc)?;

        bookmarks.push(Bookmark {
            title,
            page,
            level: 0,
        });

        cursor = dict_ref(item_dict, b"Next");
    }

    Ok(bookmarks)
}

/// Append a new top-level bookmark pointing at `page` (1-based) to the PDF outline.
pub fn add_bookmark(input: &Path, title: &str, page: u32, output: &Path) -> Result<(), EzPdfError> {
    let mut doc = load_doc(input)?;
    let page_count = doc.get_pages().len() as u32;

    if page == 0 || page > page_count {
        return Err(EzPdfError::PageOutOfRange {
            page,
            total: page_count,
        });
    }

    // Get the page object id for the destination
    let page_obj_id: ObjectId = {
        let pages = doc.get_pages();
        *pages.get(&page).ok_or(EzPdfError::PageOutOfRange {
            page,
            total: page_count,
        })?
    };

    // Ensure /Outlines root exists in the catalog
    let outlines_id = match outline_root_id(&doc) {
        Some(id) => id,
        None => {
            let id = doc.add_object(Object::Dictionary(Dictionary::from_iter(vec![
                ("Type", Object::Name(b"Outlines".to_vec())),
                ("Count", Object::Integer(0)),
            ])));
            // Link into catalog
            if let Ok(catalog) = doc.catalog_mut() {
                catalog.set("Outlines", Object::Reference(id));
            }
            id
        }
    };

    // Find the current /Last entry so we can link the new item after it
    let prev_last: Option<ObjectId> = {
        let outlines_obj = doc
            .get_object(outlines_id)
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
        let d = outlines_obj
            .as_dict()
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
        dict_ref(d, b"Last")
    };

    // Build the new outline item
    let mut item_dict = Dictionary::new();
    item_dict.set(
        "Title",
        Object::String(title.as_bytes().to_vec(), lopdf::StringFormat::Literal),
    );
    // /Dest [page_obj_id 0 R /XYZ null null null]
    item_dict.set(
        "Dest",
        Object::Array(vec![
            Object::Reference(page_obj_id),
            Object::Name(b"XYZ".to_vec()),
            Object::Null,
            Object::Null,
            Object::Null,
        ]),
    );
    item_dict.set("Parent", Object::Reference(outlines_id));

    if let Some(prev_id) = prev_last {
        item_dict.set("Prev", Object::Reference(prev_id));
    }

    let new_item_id = doc.add_object(Object::Dictionary(item_dict));

    // Update /Prev item's /Next pointer
    if let Some(prev_id) = prev_last {
        if let Ok(prev_obj) = doc.get_object_mut(prev_id) {
            if let Ok(d) = prev_obj.as_dict_mut() {
                d.set("Next", Object::Reference(new_item_id));
            }
        }
    }

    // Update /Outlines root: /Last (and /First if empty), /Count
    {
        let outlines_obj = doc
            .get_object_mut(outlines_id)
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
        let d = outlines_obj
            .as_dict_mut()
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;

        if prev_last.is_none() {
            // First bookmark ever
            d.set("First", Object::Reference(new_item_id));
        }
        d.set("Last", Object::Reference(new_item_id));

        let count = match d.get(b"Count") {
            Ok(Object::Integer(n)) => *n,
            _ => 0,
        };
        d.set("Count", Object::Integer(count + 1));
    }

    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    doc.save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

// ── helpers ────────────────────────────────────────────────────────────────

fn outline_root_id(doc: &lopdf::Document) -> Option<ObjectId> {
    let catalog = doc.catalog().ok()?;
    dict_ref(catalog, b"Outlines")
}

fn dict_ref(dict: &Dictionary, key: &[u8]) -> Option<ObjectId> {
    match dict.get(key) {
        Ok(Object::Reference(id)) => Some(*id),
        _ => None,
    }
}

fn read_pdf_string(dict: &Dictionary, key: &[u8]) -> Result<String, EzPdfError> {
    match dict.get(key) {
        Ok(Object::String(bytes, _)) => Ok(String::from_utf8_lossy(bytes).into_owned()),
        _ => Err(EzPdfError::Pdf(format!(
            "missing or invalid {} field in outline item",
            String::from_utf8_lossy(key)
        ))),
    }
}

/// Map the /Dest array's first element (page object id) to a 1-based page number.
fn dest_page_number(
    item_dict: &Dictionary,
    pages_map: &std::collections::BTreeMap<u32, ObjectId>,
    _doc: &lopdf::Document,
) -> Result<u32, EzPdfError> {
    let dest = item_dict
        .get(b"Dest")
        .map_err(|_| EzPdfError::Pdf("missing /Dest".into()))?;
    let arr = dest
        .as_array()
        .map_err(|_| EzPdfError::Pdf("/Dest is not an array".into()))?;
    let page_ref = arr
        .first()
        .ok_or_else(|| EzPdfError::Pdf("/Dest array is empty".into()))?;
    let page_id = match page_ref {
        Object::Reference(id) => *id,
        _ => return Err(EzPdfError::Pdf("/Dest[0] is not a reference".into())),
    };
    // find which page number has this object id
    for (num, &oid) in pages_map {
        if oid == page_id {
            return Ok(*num);
        }
    }
    Err(EzPdfError::Pdf(format!(
        "page object {:?} not found in page tree",
        page_id
    )))
}
