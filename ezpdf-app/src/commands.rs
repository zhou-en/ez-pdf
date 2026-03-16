use std::path::Path;

use ezpdf_core::{MetadataUpdate, PdfMetadata, WatermarkOptions};

#[tauri::command]
pub fn cmd_merge(inputs: Vec<String>, output: String) -> Result<String, String> {
    let paths: Vec<&Path> = inputs.iter().map(|s| Path::new(s.as_str())).collect();
    ezpdf_core::merge(&paths, Path::new(&output)).map_err(|e| e.to_string())?;
    Ok(format!("Merged {} files → {}", inputs.len(), output))
}

#[tauri::command]
pub fn cmd_split_range(input: String, range: String, output: String) -> Result<String, String> {
    ezpdf_core::split_range(Path::new(&input), &range, Path::new(&output))
        .map_err(|e| e.to_string())?;
    Ok(format!("Split range {} → {}", range, output))
}

#[tauri::command]
pub fn cmd_split_each(input: String, output_dir: String) -> Result<String, String> {
    ezpdf_core::split_each(Path::new(&input), Path::new(&output_dir)).map_err(|e| e.to_string())?;
    Ok(format!("Split each page → {}", output_dir))
}

#[tauri::command]
pub fn cmd_remove(input: String, pages: String, output: String) -> Result<String, String> {
    ezpdf_core::remove(Path::new(&input), &pages, Path::new(&output)).map_err(|e| e.to_string())?;
    Ok(format!("Removed pages {} → {}", pages, output))
}

#[tauri::command]
pub fn cmd_rotate(
    input: String,
    degrees: i32,
    pages: Option<String>,
    output: String,
) -> Result<String, String> {
    ezpdf_core::rotate(
        Path::new(&input),
        degrees,
        pages.as_deref(),
        Path::new(&output),
    )
    .map_err(|e| e.to_string())?;
    Ok(format!("Rotated {}° → {}", degrees, output))
}

#[tauri::command]
pub fn cmd_reorder(input: String, order: String, output: String) -> Result<String, String> {
    ezpdf_core::reorder(Path::new(&input), &order, Path::new(&output))
        .map_err(|e| e.to_string())?;
    Ok(format!("Reordered → {}", output))
}

#[tauri::command]
pub fn cmd_page_count(input: String) -> Result<u32, String> {
    ezpdf_core::page_count(Path::new(&input)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_get_metadata(input: String) -> Result<PdfMetadata, String> {
    ezpdf_core::get_metadata(Path::new(&input)).map_err(|e| e.to_string())
}

#[allow(clippy::too_many_arguments)]
#[tauri::command]
pub fn cmd_set_metadata(
    input: String,
    output: String,
    title: Option<String>,
    author: Option<String>,
    subject: Option<String>,
    keywords: Option<String>,
    creator: Option<String>,
    producer: Option<String>,
) -> Result<String, String> {
    let updates = MetadataUpdate {
        title,
        author,
        subject,
        keywords,
        creator,
        producer,
        clear_all: false,
    };
    ezpdf_core::set_metadata(Path::new(&input), updates, Path::new(&output))
        .map_err(|e| e.to_string())?;
    Ok(format!("Metadata saved → {}", output))
}

#[tauri::command]
pub fn cmd_watermark(
    input: String,
    text: String,
    font_size: f32,
    opacity: f32,
    pages: Option<String>,
    output: String,
) -> Result<String, String> {
    let opts = WatermarkOptions {
        opacity,
        color_rgb: (0.5, 0.5, 0.5),
        font_size,
        pages,
    };
    ezpdf_core::watermark(Path::new(&input), &text, opts, Path::new(&output))
        .map_err(|e| e.to_string())?;
    Ok(format!("Watermark added → {}", output))
}

#[tauri::command]
pub fn cmd_list_bookmarks(input: String) -> Result<Vec<ezpdf_core::Bookmark>, String> {
    ezpdf_core::list_bookmarks(Path::new(&input)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn cmd_add_bookmark(
    input: String,
    title: String,
    page: u32,
    output: String,
) -> Result<String, String> {
    ezpdf_core::add_bookmark(Path::new(&input), &title, page, Path::new(&output))
        .map_err(|e| e.to_string())?;
    Ok(format!("Bookmark '{}' at page {} → {}", title, page, output))
}

#[tauri::command]
pub fn cmd_extract_images(input: String, output_dir: String) -> Result<String, String> {
    let count = ezpdf_core::extract_images(Path::new(&input), Path::new(&output_dir))
        .map_err(|e| e.to_string())?;
    Ok(format!("Extracted {} image(s) → {}", count, output_dir))
}
