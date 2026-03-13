use std::collections::HashSet;
use std::path::Path;

use lopdf::{Dictionary, Object};

use crate::error::EzPdfError;
use crate::merge::load_doc;

pub fn reorder(input: &Path, order: &str, output: &Path) -> Result<(), EzPdfError> {
    let mut doc = load_doc(input)?;
    let page_count = doc.get_pages().len() as u32;

    let new_order = parse_order(order, page_count)?;

    let all_pages = doc.get_pages();
    let new_page_ids: Vec<_> = new_order
        .iter()
        .map(|&n| {
            all_pages
                .get(&n)
                .copied()
                .ok_or(EzPdfError::PageOutOfRange {
                    page: n,
                    total: page_count,
                })
        })
        .collect::<Result<_, _>>()?;

    // Build fresh pages root with new order
    let pages_root_id = doc.new_object_id();
    let kids: Vec<Object> = new_page_ids
        .iter()
        .map(|&id| Object::Reference(id))
        .collect();

    doc.objects.insert(
        pages_root_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Pages".to_vec())),
            ("Kids", Object::Array(kids)),
            ("Count", Object::Integer(page_count as i64)),
        ])),
    );

    // Update /Parent on each page
    for &page_id in &new_page_ids {
        if let Ok(page) = doc.get_object_mut(page_id) {
            if let Ok(dict) = page.as_dict_mut() {
                dict.set("Parent", Object::Reference(pages_root_id));
            }
        }
    }

    // New catalog
    let catalog_id = doc.new_object_id();
    doc.objects.insert(
        catalog_id,
        Object::Dictionary(Dictionary::from_iter(vec![
            ("Type", Object::Name(b"Catalog".to_vec())),
            ("Pages", Object::Reference(pages_root_id)),
        ])),
    );
    doc.trailer.set("Root", Object::Reference(catalog_id));

    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    doc.save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

fn parse_order(order: &str, page_count: u32) -> Result<Vec<u32>, EzPdfError> {
    let pages: Vec<u32> = order
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<u32>()
                .map_err(|_| EzPdfError::InvalidSyntax {
                    input: order.to_string(),
                    hint: format!("'{s}' is not a valid page number"),
                })
        })
        .collect::<Result<_, _>>()?;

    // Validate no zeros
    if pages.contains(&0) {
        return Err(EzPdfError::InvalidSyntax {
            input: order.to_string(),
            hint: "page numbers are 1-indexed; 0 is not valid".to_string(),
        });
    }

    // Validate no out-of-range
    for &p in &pages {
        if p > page_count {
            return Err(EzPdfError::PageOutOfRange {
                page: p,
                total: page_count,
            });
        }
    }

    // Validate no duplicates
    let unique: HashSet<_> = pages.iter().collect();
    if unique.len() != pages.len() {
        return Err(EzPdfError::InvalidSyntax {
            input: order.to_string(),
            hint: "order contains duplicate page numbers".to_string(),
        });
    }

    // Validate covers all pages (length must equal page_count)
    if pages.len() != page_count as usize {
        return Err(EzPdfError::InvalidSyntax {
            input: order.to_string(),
            hint: format!(
                "order must include all {} pages; got {}",
                page_count,
                pages.len()
            ),
        });
    }

    Ok(pages)
}
