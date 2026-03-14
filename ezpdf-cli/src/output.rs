use std::path::{Path, PathBuf};

use ezpdf_core::error::EzPdfError;
use indicatif::{ProgressBar, ProgressStyle};

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
