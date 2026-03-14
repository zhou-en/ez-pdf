use std::path::Path;

use lopdf::{Dictionary, Object, Stream};

use crate::error::EzPdfError;
use crate::merge::load_doc;
use crate::page_range;

pub struct WatermarkOptions {
    /// Opacity (0.0 transparent – 1.0 opaque)
    pub opacity: f32,
    /// RGB colour components (each 0.0–1.0)
    pub color_rgb: (f32, f32, f32),
    /// Font size in points
    pub font_size: f32,
    /// Page range (e.g. "1,3,5-7"). None = all pages.
    pub pages: Option<String>,
}

pub fn watermark(
    input: &Path,
    text: &str,
    opts: WatermarkOptions,
    output: &Path,
) -> Result<(), EzPdfError> {
    let mut doc = load_doc(input)?;
    let page_count = doc.get_pages().len() as u32;

    let target_pages: Vec<u32> = match opts.pages {
        Some(ref range) => page_range::parse(range, page_count)?,
        None => (1..=page_count).collect(),
    };

    // Collect page IDs to avoid borrow conflicts
    let page_ids: Vec<_> = {
        let all = doc.get_pages();
        target_pages
            .iter()
            .filter_map(|n| all.get(n).copied())
            .collect()
    };

    // Register a standard Type1 Helvetica font object once
    let font_id = doc.add_object(Object::Dictionary(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"Font".to_vec())),
        ("Subtype", Object::Name(b"Type1".to_vec())),
        ("BaseFont", Object::Name(b"Helvetica".to_vec())),
    ])));

    let (r, g, b) = opts.color_rgb;
    let draw = WatermarkDraw {
        text,
        opacity: opts.opacity,
        r,
        g,
        b,
        font_size: opts.font_size,
    };

    for page_id in page_ids {
        let stream_id = add_watermark_stream(&mut doc, &draw, font_id)?;
        append_content_stream(&mut doc, page_id, stream_id, font_id)?;
    }

    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    doc.save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

struct WatermarkDraw<'a> {
    text: &'a str,
    opacity: f32,
    r: f32,
    g: f32,
    b: f32,
    font_size: f32,
}

/// Build and add a new content stream that draws a diagonal watermark.
fn add_watermark_stream(
    doc: &mut lopdf::Document,
    draw: &WatermarkDraw<'_>,
    _font_id: lopdf::ObjectId,
) -> Result<lopdf::ObjectId, EzPdfError> {
    let WatermarkDraw {
        text,
        opacity,
        r,
        g,
        b,
        font_size,
    } = *draw;

    // Approximate Helvetica character width ≈ 0.55 × font_size per character
    let text_width = text.len() as f32 * font_size * 0.55;
    let x_offset = -text_width / 2.0;

    // Standard letter page centre ≈ (306, 396). Rotate 45°.
    let content = format!(
        "q\n\
         /GS1 gs\n\
         {r:.3} {g:.3} {b:.3} rg\n\
         1 0 0 1 306 396 cm\n\
         0.707 0.707 -0.707 0.707 0 0 cm\n\
         BT\n\
         /Helvetica {font_size:.1} Tf\n\
         {x_offset:.1} 0 Td\n\
         ({text}) Tj\n\
         ET\n\
         Q\n",
    );

    // Extended graphics state for opacity
    let gs_id = doc.add_object(Object::Dictionary(Dictionary::from_iter(vec![
        ("Type", Object::Name(b"ExtGState".to_vec())),
        ("ca", Object::Real(opacity)),
        ("CA", Object::Real(opacity)),
    ])));

    let mut stream_dict = Dictionary::new();
    stream_dict.set(
        "Resources",
        Object::Dictionary(Dictionary::from_iter(vec![(
            "ExtGState",
            Object::Dictionary(Dictionary::from_iter(vec![(
                "GS1",
                Object::Reference(gs_id),
            )])),
        )])),
    );

    let stream_id = doc.add_object(Stream::new(stream_dict, content.into_bytes()));
    Ok(stream_id)
}

/// Append `stream_id` to the page's /Contents array and add Helvetica to /Resources/Font.
fn append_content_stream(
    doc: &mut lopdf::Document,
    page_id: lopdf::ObjectId,
    stream_id: lopdf::ObjectId,
    font_id: lopdf::ObjectId,
) -> Result<(), EzPdfError> {
    // Collect existing content references before mutably borrowing page
    let existing_contents: Vec<Object> = {
        if let Ok(page_obj) = doc.get_object(page_id) {
            if let Ok(dict) = page_obj.as_dict() {
                match dict.get(b"Contents").ok() {
                    Some(Object::Array(arr)) => arr.clone(),
                    Some(Object::Reference(id)) => vec![Object::Reference(*id)],
                    _ => vec![],
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        }
    };

    let mut new_contents = existing_contents;
    new_contents.push(Object::Reference(stream_id));

    // Mutably update the page dictionary
    if let Ok(page_obj) = doc.get_object_mut(page_id) {
        if let Ok(dict) = page_obj.as_dict_mut() {
            dict.set("Contents", Object::Array(new_contents));

            // Add Helvetica to /Resources/Font
            let font_ref = Object::Reference(font_id);
            match dict.get_mut(b"Resources") {
                Ok(res_obj) => {
                    if let Ok(res_dict) = res_obj.as_dict_mut() {
                        match res_dict.get_mut(b"Font") {
                            Ok(font_obj) => {
                                if let Ok(font_dict) = font_obj.as_dict_mut() {
                                    font_dict.set("Helvetica", font_ref);
                                }
                            }
                            Err(_) => {
                                res_dict.set(
                                    "Font",
                                    Object::Dictionary(Dictionary::from_iter(vec![(
                                        "Helvetica",
                                        font_ref,
                                    )])),
                                );
                            }
                        }
                    }
                }
                Err(_) => {
                    dict.set(
                        "Resources",
                        Object::Dictionary(Dictionary::from_iter(vec![(
                            "Font",
                            Object::Dictionary(Dictionary::from_iter(vec![(
                                "Helvetica",
                                font_ref,
                            )])),
                        )])),
                    );
                }
            }
        }
    }

    Ok(())
}
