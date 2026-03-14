use std::path::{Path, PathBuf};

use ezpdf_core::{error::EzPdfError, load_doc_with_password};
use indicatif::{ProgressBar, ProgressStyle};
use tempfile::NamedTempFile;

/// Read password from a file (strips trailing whitespace).
pub fn read_password_file(path: &std::path::Path) -> anyhow::Result<String> {
    let raw = std::fs::read_to_string(path)?;
    Ok(raw.trim_end().to_string())
}

/// Resolve effective password: `--password` takes precedence over `--password-file`.
pub fn resolve_password(
    password: Option<&str>,
    password_file: Option<&std::path::Path>,
) -> anyhow::Result<Option<String>> {
    if let Some(pw) = password {
        return Ok(Some(pw.to_string()));
    }
    if let Some(pf) = password_file {
        return Ok(Some(read_password_file(pf)?));
    }
    Ok(None)
}

/// If `password` is provided, load and decrypt the PDF, save to a temp file, and
/// return `(temp_path, Some(temp_file))`. The caller must keep the `NamedTempFile`
/// alive until the operation is complete (drop = file deleted).
/// If no password, returns `(original_path, None)` with no I/O.
pub fn resolve_input(
    path: &Path,
    password: Option<&str>,
) -> anyhow::Result<(PathBuf, Option<NamedTempFile>)> {
    let pw = match password {
        Some(p) => p,
        None => return Ok((path.to_path_buf(), None)),
    };

    let mut doc = load_doc_with_password(path, Some(pw))?;
    let tmp = NamedTempFile::new()?;
    {
        let mut f = std::fs::File::create(tmp.path())?;
        doc.save_to(&mut f)
            .map_err(|e| anyhow::anyhow!("failed to save decrypted PDF: {e}"))?;
    }
    Ok((tmp.path().to_path_buf(), Some(tmp)))
}

pub fn print_success(msg: &str, quiet: bool) {
    if !quiet {
        println!("{msg}");
    }
}

/// Apply `op` to each input independently, saving to `out_dir/<original_filename>`.
pub fn run_batch_independent<F>(
    inputs: Vec<PathBuf>,
    out_dir: &Path,
    quiet: bool,
    label: &str,
    op: F,
) -> anyhow::Result<()>
where
    F: Fn(&Path, &Path) -> Result<(), EzPdfError>,
{
    std::fs::create_dir_all(out_dir)?;
    for path in &inputs {
        let out = out_dir.join(path.file_name().unwrap());
        op(path, &out)?;
    }
    if !quiet {
        println!("{label} — {} files → {}", inputs.len(), out_dir.display());
    }
    Ok(())
}

/// Returns a progress bar if page_count > 20 and quiet is false, otherwise None.
pub fn maybe_progress(label: &str, page_count: u32, quiet: bool) -> Option<ProgressBar> {
    if quiet || page_count <= 20 {
        return None;
    }
    let pb = ProgressBar::new(page_count as u64);
    pb.set_style(
        ProgressStyle::with_template("[{bar:40}] Processing page {pos}/{len} — {msg}")
            .unwrap()
            .progress_chars("█░"),
    );
    pb.set_message(label.to_string());
    Some(pb)
}
