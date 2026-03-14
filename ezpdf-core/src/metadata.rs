use std::path::Path;

use lopdf::{Dictionary, Object, ObjectId};
use serde::Serialize;

use crate::error::EzPdfError;
use crate::merge::load_doc;

#[derive(Debug, PartialEq, Serialize)]
pub struct PdfMetadata {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
}

#[derive(Debug, Default)]
pub struct MetadataUpdate {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub clear_all: bool,
}

pub fn get_metadata(input: &Path) -> Result<PdfMetadata, EzPdfError> {
    let doc = load_doc(input)?;
    let info_dict = get_info_dict(&doc);

    Ok(PdfMetadata {
        title: info_dict.as_ref().and_then(|d| pdf_string(d, b"Title")),
        author: info_dict.as_ref().and_then(|d| pdf_string(d, b"Author")),
        subject: info_dict.as_ref().and_then(|d| pdf_string(d, b"Subject")),
        keywords: info_dict.as_ref().and_then(|d| pdf_string(d, b"Keywords")),
        creator: info_dict.as_ref().and_then(|d| pdf_string(d, b"Creator")),
        producer: info_dict.as_ref().and_then(|d| pdf_string(d, b"Producer")),
    })
}

pub fn set_metadata(
    input: &Path,
    updates: MetadataUpdate,
    output: &Path,
) -> Result<(), EzPdfError> {
    let mut doc = load_doc(input)?;

    let info_id = get_or_create_info_id(&mut doc)?;

    let info_obj = doc
        .get_object_mut(info_id)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
    let info_dict = info_obj
        .as_dict_mut()
        .map_err(|e| EzPdfError::Pdf(e.to_string()))?;

    if updates.clear_all {
        info_dict.remove(b"Title");
        info_dict.remove(b"Author");
        info_dict.remove(b"Subject");
        info_dict.remove(b"Keywords");
        info_dict.remove(b"Creator");
        info_dict.remove(b"Producer");
    }

    if let Some(v) = updates.title {
        info_dict.set("Title", Object::string_literal(v));
    }
    if let Some(v) = updates.author {
        info_dict.set("Author", Object::string_literal(v));
    }
    if let Some(v) = updates.subject {
        info_dict.set("Subject", Object::string_literal(v));
    }
    if let Some(v) = updates.keywords {
        info_dict.set("Keywords", Object::string_literal(v));
    }
    if let Some(v) = updates.creator {
        info_dict.set("Creator", Object::string_literal(v));
    }
    if let Some(v) = updates.producer {
        info_dict.set("Producer", Object::string_literal(v));
    }

    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    doc.save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

/// Follow the trailer /Info reference and return its dictionary, if any.
fn get_info_dict(doc: &lopdf::Document) -> Option<Dictionary> {
    let info_ref = doc.trailer.get(b"Info").ok()?;
    let info_id = info_ref.as_reference().ok()?;
    doc.get_object(info_id).ok()?.as_dict().ok().cloned()
}

/// Return the ObjectId of the Info dictionary, creating one if it doesn't exist.
fn get_or_create_info_id(doc: &mut lopdf::Document) -> Result<ObjectId, EzPdfError> {
    // Try to find existing Info reference in trailer
    if let Ok(obj) = doc.trailer.get(b"Info") {
        if let Ok(id) = obj.as_reference() {
            return Ok(id);
        }
    }

    // Create a new empty Info dictionary and wire it into the trailer
    let info_id = doc.add_object(Object::Dictionary(Dictionary::new()));
    doc.trailer.set("Info", Object::Reference(info_id));
    Ok(info_id)
}

/// Decode a PDF string value from a dictionary key (UTF-8 or UTF-16BE).
fn pdf_string(dict: &Dictionary, key: &[u8]) -> Option<String> {
    let obj = dict.get(key).ok()?;
    let bytes = match obj {
        Object::String(b, _) => b.clone(),
        _ => return None,
    };
    if bytes.starts_with(&[0xFE, 0xFF]) {
        let pairs: Vec<u16> = bytes[2..]
            .chunks(2)
            .map(|c| {
                if c.len() == 2 {
                    u16::from_be_bytes([c[0], c[1]])
                } else {
                    0
                }
            })
            .collect();
        String::from_utf16(&pairs).ok()
    } else {
        String::from_utf8(bytes).ok()
    }
}
