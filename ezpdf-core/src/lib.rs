#[cfg(not(target_arch = "wasm32"))]
pub mod batch;
pub mod bookmarks;
pub mod error;
pub mod images;
pub mod info;
#[cfg(feature = "markdown")]
pub mod markdown;
pub mod merge;
pub mod metadata;
pub mod optimize;
pub mod page_range;
pub mod remove;
pub mod reorder;
pub mod rotate;
pub mod split;
pub mod watermark;

// Re-export watermark module under an alias so tests can access WatermarkOptions
pub use watermark as watermark_mod;

pub use bookmarks::{add_bookmark, list_bookmarks, Bookmark};
pub use images::extract_images;
pub use info::{info, info_bytes, page_count, page_count_bytes, PdfInfo};
#[cfg(feature = "markdown")]
pub use markdown::{markdown, to_markdown, to_markdown_bytes, MarkdownOptions};
pub use merge::{load_doc_mem, load_doc_with_password, merge, merge_bytes};
pub use metadata::{get_metadata, set_metadata, MetadataUpdate, PdfMetadata};
pub use optimize::{optimize, OptimizeStats};
pub use remove::{remove, remove_bytes};
pub use reorder::reorder;
pub use rotate::{rotate, rotate_bytes};
pub use split::{split_each, split_each_bytes, split_range, split_range_bytes};
pub use watermark::{watermark, WatermarkOptions};
