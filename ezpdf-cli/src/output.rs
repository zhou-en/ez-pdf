use indicatif::{ProgressBar, ProgressStyle};

pub fn print_success(msg: &str, quiet: bool) {
    if !quiet {
        println!("{msg}");
    }
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
