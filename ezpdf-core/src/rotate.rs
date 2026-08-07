use std::path::Path;

use lopdf::Object;

use crate::error::EzPdfError;
use crate::merge::{load_doc, load_doc_mem, save_to_vec};
use crate::page_range;

pub fn rotate(
    input: &Path,
    degrees: i32,
    pages: Option<&str>,
    output: &Path,
) -> Result<(), EzPdfError> {
    let mut doc = build_rotated(load_doc(input)?, degrees, pages)?;
    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    doc.save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

/// [`rotate`] over an in-memory PDF.
pub fn rotate_bytes(
    input: &[u8],
    degrees: i32,
    pages: Option<&str>,
) -> Result<Vec<u8>, EzPdfError> {
    // normalize_degrees runs before the parse so an invalid angle is reported
    // even when the document itself cannot be opened.
    let normalized = normalize_degrees(degrees)?;
    save_to_vec(build_rotated(
        load_doc_mem(input, None)?,
        normalized,
        pages,
    )?)
}

/// Applies a relative rotation to the selected pages.
fn build_rotated(
    mut doc: lopdf::Document,
    degrees: i32,
    pages: Option<&str>,
) -> Result<lopdf::Document, EzPdfError> {
    let normalized = normalize_degrees(degrees)?;
    let page_count = doc.get_pages().len() as u32;

    let target_pages: Vec<u32> = match pages {
        Some(range) => page_range::parse(range, page_count)?,
        None => (1..=page_count).collect(),
    };

    // Collect page object IDs first to avoid borrow conflicts
    let page_ids: Vec<_> = {
        let all_pages = doc.get_pages();
        target_pages
            .iter()
            .filter_map(|n| all_pages.get(n).copied())
            .collect()
    };

    for page_id in page_ids {
        if let Ok(page) = doc.get_object_mut(page_id) {
            if let Ok(dict) = page.as_dict_mut() {
                let current = dict.get(b"Rotate").and_then(|r| r.as_i64()).unwrap_or(0) as i32;
                let new_rotation = ((current + normalized).rem_euclid(360)) as i64;
                dict.set("Rotate", Object::Integer(new_rotation));
            }
        }
    }

    Ok(doc)
}

/// Normalize degrees to a multiple of 90 in [0, 360).
/// Returns InvalidSyntax if degrees is not a multiple of 90.
fn normalize_degrees(degrees: i32) -> Result<i32, EzPdfError> {
    if degrees.rem_euclid(90) != 0 {
        return Err(EzPdfError::InvalidSyntax {
            input: degrees.to_string(),
            hint: "rotation must be a multiple of 90 (e.g. 90, 180, 270, -90)".to_string(),
        });
    }
    Ok(degrees)
}
