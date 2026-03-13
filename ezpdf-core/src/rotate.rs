use std::path::Path;

use lopdf::Object;

use crate::error::EzPdfError;
use crate::merge::load_doc;
use crate::page_range;

pub fn rotate(
    input: &Path,
    degrees: i32,
    pages: Option<&str>,
    output: &Path,
) -> Result<(), EzPdfError> {
    let normalized = normalize_degrees(degrees)?;

    let mut doc = load_doc(input)?;
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

    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    doc.save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
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
