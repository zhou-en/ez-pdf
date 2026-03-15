use std::path::Path;

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
