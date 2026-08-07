//! WebAssembly bindings for `ezpdf-core`.
//!
//! Every export is a thin shim over a `*_bytes` function: PDFs arrive as a
//! `Uint8Array` from JS and leave the same way, so nothing here touches a
//! filesystem. `EzPdfError` is flattened to a JS exception whose message keeps
//! the remedy text the core produces (e.g. `NoTextLayer` naming `ocrmypdf`).
use wasm_bindgen::prelude::*;

use ezpdf_core::error::EzPdfError;

fn js(err: EzPdfError) -> JsError {
    JsError::new(&err.to_string())
}

/// Number of pages in a PDF.
#[wasm_bindgen]
pub fn page_count(bytes: &[u8]) -> Result<u32, JsError> {
    ezpdf_core::page_count_bytes(bytes).map_err(js)
}

/// Page count, per-page dimensions and metadata, as a JSON string.
///
/// The page grid needs real page dimensions to render true-aspect tiles.
#[wasm_bindgen]
pub fn info_json(bytes: &[u8]) -> Result<String, JsError> {
    let info = ezpdf_core::info_bytes(bytes).map_err(js)?;
    serde_json::to_string(&info).map_err(|e| JsError::new(&e.to_string()))
}

/// Concatenates several PDFs in the given order.
#[wasm_bindgen]
pub fn merge(docs: Vec<js_sys::Uint8Array>) -> Result<Vec<u8>, JsError> {
    let owned: Vec<Vec<u8>> = docs.iter().map(|d| d.to_vec()).collect();
    let refs: Vec<&[u8]> = owned.iter().map(|d| d.as_slice()).collect();
    ezpdf_core::merge_bytes(&refs).map_err(js)
}

/// Extracts a page range such as `1-5,7`.
#[wasm_bindgen]
pub fn split_range(bytes: &[u8], range: &str) -> Result<Vec<u8>, JsError> {
    ezpdf_core::split_range_bytes(bytes, range).map_err(js)
}

/// Bursts a PDF into one document per page.
#[wasm_bindgen]
pub fn split_each(bytes: &[u8]) -> Result<Vec<js_sys::Uint8Array>, JsError> {
    Ok(ezpdf_core::split_each_bytes(bytes)
        .map_err(js)?
        .iter()
        .map(|page| js_sys::Uint8Array::from(page.as_slice()))
        .collect())
}

/// Deletes the given pages.
#[wasm_bindgen]
pub fn remove(bytes: &[u8], pages: &str) -> Result<Vec<u8>, JsError> {
    ezpdf_core::remove_bytes(bytes, pages).map_err(js)
}

/// Rotates all pages, or only `pages` when supplied.
#[wasm_bindgen]
pub fn rotate(bytes: &[u8], degrees: i32, pages: Option<String>) -> Result<Vec<u8>, JsError> {
    ezpdf_core::rotate_bytes(bytes, degrees, pages.as_deref()).map_err(js)
}

/// Converts a PDF to Markdown.
///
/// Only present in the `markdown` feature build — it pulls in `pdf-inspector`,
/// which is ~25x the size of the rest of the module put together.
#[cfg(feature = "markdown")]
#[wasm_bindgen]
pub fn to_markdown(bytes: &[u8], page_breaks: bool) -> Result<String, JsError> {
    let options = ezpdf_core::MarkdownOptions {
        pages: None,
        page_breaks,
    };
    ezpdf_core::to_markdown_bytes(bytes, &options).map_err(js)
}
