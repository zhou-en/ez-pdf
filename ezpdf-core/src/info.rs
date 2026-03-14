use std::path::Path;

use lopdf::Object;
use serde::Serialize;

use crate::error::EzPdfError;
use crate::merge::load_doc;

#[derive(Debug, PartialEq, Serialize)]
pub struct PdfInfo {
    pub page_count: u32,
    pub dimensions: Vec<(f64, f64)>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
}

pub fn info(input: &Path) -> Result<PdfInfo, EzPdfError> {
    let doc = load_doc(input)?;

    let page_count = doc.get_pages().len() as u32;

    // Collect dimensions in page-number order
    let mut pages: Vec<(u32, lopdf::ObjectId)> = doc.get_pages().into_iter().collect();
    pages.sort_by_key(|(n, _)| *n);

    let mut dimensions = Vec::with_capacity(pages.len() as usize);
    for (_, page_id) in &pages {
        let (w, h) = page_dimensions(&doc, *page_id);
        dimensions.push((w, h));
    }

    // Extract document metadata from the Info dictionary
    let info_dict = get_info_dict(&doc);

    Ok(PdfInfo {
        page_count,
        dimensions,
        title: info_dict.as_ref().and_then(|d| pdf_string(d, b"Title")),
        author: info_dict.as_ref().and_then(|d| pdf_string(d, b"Author")),
        subject: info_dict.as_ref().and_then(|d| pdf_string(d, b"Subject")),
        keywords: info_dict.as_ref().and_then(|d| pdf_string(d, b"Keywords")),
        creator: info_dict.as_ref().and_then(|d| pdf_string(d, b"Creator")),
        producer: info_dict.as_ref().and_then(|d| pdf_string(d, b"Producer")),
    })
}

/// Read the /MediaBox from a page dict, walking up to /Pages parent if needed.
fn page_dimensions(doc: &lopdf::Document, page_id: lopdf::ObjectId) -> (f64, f64) {
    // Walk up the page tree to find /MediaBox
    let mut current_id = Some(page_id);
    while let Some(id) = current_id {
        if let Ok(dict) = doc.get_object(id).and_then(|o| o.as_dict()) {
            if let Ok(media_box) = dict.get(b"MediaBox").and_then(|o| o.as_array()) {
                let coords: Vec<f64> = media_box
                    .iter()
                    .map(|o| match o {
                        Object::Integer(i) => *i as f64,
                        Object::Real(r) => *r as f64,
                        _ => 0.0,
                    })
                    .collect();
                if coords.len() == 4 {
                    let w = (coords[2] - coords[0]).abs();
                    let h = (coords[3] - coords[1]).abs();
                    return (w, h);
                }
            }
            // Walk up to parent
            current_id = dict
                .get(b"Parent")
                .ok()
                .and_then(|o| o.as_reference().ok());
        } else {
            break;
        }
    }
    (0.0, 0.0)
}

/// Resolve the Info dictionary from the document trailer.
fn get_info_dict(doc: &lopdf::Document) -> Option<lopdf::Dictionary> {
    let info_ref = doc.trailer.get(b"Info").ok()?;
    let info_id = info_ref.as_reference().ok()?;
    doc.get_object(info_id)
        .ok()?
        .as_dict()
        .ok()
        .cloned()
}

/// Read a PDF string field from a dictionary, decoding bytes as UTF-8 best-effort.
fn pdf_string(dict: &lopdf::Dictionary, key: &[u8]) -> Option<String> {
    let obj = dict.get(key).ok()?;
    let bytes = match obj {
        Object::String(b, _) => b.clone(),
        _ => return None,
    };
    // UTF-16BE with BOM \xFE\xFF
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let pairs: Vec<u16> = bytes[2..]
            .chunks(2)
            .map(|c| if c.len() == 2 { u16::from_be_bytes([c[0], c[1]]) } else { 0 })
            .collect();
        String::from_utf16(&pairs).ok()
    } else {
        String::from_utf8(bytes).ok()
    }
}
