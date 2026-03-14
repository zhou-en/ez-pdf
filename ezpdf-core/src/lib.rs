pub mod batch;
pub mod bookmarks;
pub mod error;
pub mod info;
pub mod merge;
pub mod metadata;
pub mod page_range;
pub mod remove;
pub mod reorder;
pub mod rotate;
pub mod split;
pub mod watermark;

// Re-export watermark module under an alias so tests can access WatermarkOptions
pub use watermark as watermark_mod;

pub use bookmarks::{add_bookmark, list_bookmarks, Bookmark};
pub use info::{info, PdfInfo};
pub use merge::{load_doc_with_password, merge};
pub use metadata::{get_metadata, set_metadata};
pub use remove::remove;
pub use reorder::reorder;
pub use rotate::rotate;
pub use split::{split_each, split_range};
pub use watermark::watermark;
