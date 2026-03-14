use std::path::{Path, PathBuf};

use crate::error::EzPdfError;

pub fn collect_pdf_inputs(dir: &Path) -> Result<Vec<PathBuf>, EzPdfError> {
    let mut paths: Vec<PathBuf> = std::fs::read_dir(dir)
        .map_err(EzPdfError::Io)?
        .filter_map(|entry| entry.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|e| e.to_str()) == Some("pdf"))
        .collect();

    paths.sort_by(|a, b| a.file_name().cmp(&b.file_name()));
    Ok(paths)
}
