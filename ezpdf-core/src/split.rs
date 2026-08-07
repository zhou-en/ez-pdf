use std::path::Path;

use crate::error::EzPdfError;
use crate::merge::{load_doc, load_doc_mem, save_to_vec};
use crate::page_range;
use crate::remove::build_kept;

pub fn split_range(input: &Path, range: &str, output: &Path) -> Result<(), EzPdfError> {
    if output.is_dir() {
        return Err(EzPdfError::Io(std::io::Error::other(format!(
            "'{}' already exists as a directory — delete it or run Burst mode to recreate",
            output.display()
        ))));
    }
    let doc = load_doc(input)?;
    let page_count = doc.get_pages().len() as u32;
    let pages = page_range::parse(range, page_count)?;

    let mut result = build_kept(doc, &pages)?;
    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    result
        .save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))
}

pub fn split_each(input: &Path, output_dir: &Path) -> Result<(), EzPdfError> {
    std::fs::create_dir_all(output_dir).map_err(EzPdfError::Io)?;

    let doc = load_doc(input)?;
    let page_count = doc.get_pages().len() as u32;
    let digits = digits_needed(page_count);

    for page_num in 1..=page_count {
        let filename = format!("page-{page_num:0>width$}.pdf", width = digits);
        let out_path = output_dir.join(filename);

        let mut page_doc = build_kept(doc.clone(), &[page_num])?;
        let mut file = std::fs::File::create(&out_path).map_err(EzPdfError::Io)?;
        page_doc
            .save_to(&mut file)
            .map_err(|e| EzPdfError::Pdf(e.to_string()))?;
    }

    Ok(())
}

/// [`split_range`] over an in-memory PDF.
pub fn split_range_bytes(input: &[u8], range: &str) -> Result<Vec<u8>, EzPdfError> {
    let doc = load_doc_mem(input, None)?;
    let page_count = doc.get_pages().len() as u32;
    let pages = page_range::parse(range, page_count)?;

    save_to_vec(build_kept(doc, &pages)?)
}

/// [`split_each`] over an in-memory PDF.
///
/// Returns one document per page in page order. The path variant writes these
/// to numbered files; callers without a filesystem get the bytes instead.
pub fn split_each_bytes(input: &[u8]) -> Result<Vec<Vec<u8>>, EzPdfError> {
    let doc = load_doc_mem(input, None)?;
    let page_count = doc.get_pages().len() as u32;

    (1..=page_count)
        .map(|page_num| save_to_vec(build_kept(doc.clone(), &[page_num])?))
        .collect()
}

/// Returns the number of decimal digits needed to represent `n` (minimum 1).
fn digits_needed(n: u32) -> usize {
    if n == 0 {
        1
    } else {
        n.to_string().len()
    }
}
