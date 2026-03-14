// lopdf 0.31 provides `Document::prune_objects()` via the `Processor` trait which
// is implemented on `Document`. It traverses all objects reachable from the trailer
// and removes any orphaned objects not in the reachable set.
// We call it here and report back the count and estimated bytes saved.

use std::path::Path;

use crate::error::EzPdfError;
use crate::merge::load_doc;

/// Statistics returned by `optimize`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptimizeStats {
    /// Number of unreferenced objects removed.
    pub objects_removed: u32,
    /// Estimated bytes saved (size_before − size_after).
    pub bytes_saved: i64,
}

/// Remove unreferenced objects from `input` and write the cleaned PDF to `output`.
pub fn optimize(input: &Path, output: &Path) -> Result<OptimizeStats, EzPdfError> {
    let mut doc = load_doc(input)?;

    let size_before = std::fs::metadata(input)
        .map(|m| m.len() as i64)
        .unwrap_or(0);

    // prune_objects() removes all objects not reachable from the trailer
    // and returns the list of removed ObjectIds.
    let removed = doc.prune_objects();
    let objects_removed = removed.len() as u32;

    let mut file = std::fs::File::create(output).map_err(EzPdfError::Io)?;
    doc.save_to(&mut file)
        .map_err(|e| EzPdfError::Pdf(e.to_string()))?;

    let size_after = std::fs::metadata(output)
        .map(|m| m.len() as i64)
        .unwrap_or(0);

    Ok(OptimizeStats {
        objects_removed,
        bytes_saved: size_before - size_after,
    })
}
