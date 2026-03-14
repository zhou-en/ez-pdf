use std::io::{Read, Write};
use std::path::Path;

use flate2::read::ZlibDecoder;
use lopdf::Object;

use crate::error::EzPdfError;
use crate::merge::load_doc;

/// Extract all embedded images from `input` into `output_dir`.
/// JPEG images are saved as `.jpg`; FlateDecode (raw pixel) images are saved as `.png`.
/// Returns the total count of images extracted.
pub fn extract_images(input: &Path, output_dir: &Path) -> Result<u32, EzPdfError> {
    let doc = load_doc(input)?;
    std::fs::create_dir_all(output_dir).map_err(EzPdfError::Io)?;

    let mut count: u32 = 0;

    for (page_num, _page_id) in doc.get_pages() {
        // Get the page object to access /Resources
        let resources = match page_resources(&doc, page_num) {
            Some(r) => r,
            None => continue,
        };

        // Collect XObject references to avoid borrow issues
        let xobject_refs: Vec<(Vec<u8>, lopdf::ObjectId)> = {
            match resources.get(b"XObject") {
                Ok(Object::Dictionary(xobj_dict)) => xobj_dict
                    .iter()
                    .filter_map(|(k, v)| match v {
                        Object::Reference(id) => Some((k.clone(), *id)),
                        _ => None,
                    })
                    .collect(),
                Ok(Object::Reference(id)) => {
                    if let Ok(Object::Dictionary(d)) = doc.get_object(*id) {
                        d.iter()
                            .filter_map(|(k, v)| match v {
                                Object::Reference(id) => Some((k.clone(), *id)),
                                _ => None,
                            })
                            .collect()
                    } else {
                        vec![]
                    }
                }
                _ => continue,
            }
        };

        let mut img_index: u32 = 1;
        for (_name, obj_id) in xobject_refs {
            let stream = match doc.get_object(obj_id) {
                Ok(Object::Stream(s)) => s,
                _ => continue,
            };

            // Must be /Subtype /Image
            let subtype = match stream.dict.get(b"Subtype") {
                Ok(Object::Name(n)) => n.clone(),
                _ => continue,
            };
            if subtype != b"Image" {
                continue;
            }

            let filter: Option<Vec<u8>> = match stream.dict.get(b"Filter") {
                Ok(Object::Name(n)) => Some(n.clone()),
                Ok(Object::Array(arr)) => arr.first().and_then(|v| {
                    if let Object::Name(n) = v {
                        Some(n.clone())
                    } else {
                        None
                    }
                }),
                _ => None,
            };

            let raw = &stream.content;

            if filter.as_deref() == Some(b"DCTDecode") {
                let filename = format!("page-{page_num}-image-{img_index}.jpg");
                let out_path = output_dir.join(&filename);
                let mut file = std::fs::File::create(&out_path).map_err(EzPdfError::Io)?;
                file.write_all(raw).map_err(EzPdfError::Io)?;
                count += 1;
                img_index += 1;
            } else if filter.as_deref() == Some(b"FlateDecode") {
                let width = dict_u32(&stream.dict, b"Width")?;
                let height = dict_u32(&stream.dict, b"Height")?;
                let channels = color_channels(&stream.dict);

                let mut decoder = ZlibDecoder::new(raw.as_slice());
                let mut pixels = Vec::new();
                decoder.read_to_end(&mut pixels).map_err(EzPdfError::Io)?;

                let filename = format!("page-{page_num}-image-{img_index}.png");
                let out_path = output_dir.join(&filename);
                write_png(&out_path, width, height, channels, &pixels)?;
                count += 1;
                img_index += 1;
            }
            // Other filters (LZWDecode, etc.) are skipped
        }
    }

    Ok(count)
}

// ── helpers ────────────────────────────────────────────────────────────────

fn page_resources(doc: &lopdf::Document, page_num: u32) -> Option<lopdf::Dictionary> {
    let page_id = doc.get_pages().get(&page_num).copied()?;
    let page_obj = doc.get_object(page_id).ok()?;
    let page_dict = page_obj.as_dict().ok()?;

    match page_dict.get(b"Resources") {
        Ok(Object::Dictionary(d)) => Some(d.clone()),
        Ok(Object::Reference(id)) => {
            if let Ok(Object::Dictionary(d)) = doc.get_object(*id) {
                Some(d.clone())
            } else {
                None
            }
        }
        _ => None,
    }
}

fn dict_u32(dict: &lopdf::Dictionary, key: &[u8]) -> Result<u32, EzPdfError> {
    match dict.get(key) {
        Ok(Object::Integer(n)) => Ok(*n as u32),
        _ => Err(EzPdfError::Pdf(format!(
            "missing or invalid {} in image dict",
            String::from_utf8_lossy(key)
        ))),
    }
}

fn color_channels(dict: &lopdf::Dictionary) -> u8 {
    match dict.get(b"ColorSpace") {
        Ok(Object::Name(n)) => match n.as_slice() {
            b"DeviceRGB" => 3,
            b"DeviceCMYK" => 4,
            _ => 1, // DeviceGray or indexed
        },
        _ => 3,
    }
}

fn write_png(
    path: &Path,
    width: u32,
    height: u32,
    channels: u8,
    pixels: &[u8],
) -> Result<(), EzPdfError> {
    let file = std::fs::File::create(path).map_err(EzPdfError::Io)?;
    let mut encoder = png::Encoder::new(file, width, height);
    let color_type = if channels == 3 {
        png::ColorType::Rgb
    } else if channels == 4 {
        png::ColorType::Rgba
    } else {
        png::ColorType::Grayscale
    };
    encoder.set_color(color_type);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder
        .write_header()
        .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
    writer
        .write_image_data(pixels)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}
