pub mod error;
pub mod info;
pub mod merge;
pub mod page_range;
pub mod remove;
pub mod reorder;
pub mod rotate;
pub mod split;

pub use info::{info, PdfInfo};
pub use merge::merge;
pub use remove::remove;
pub use reorder::reorder;
pub use rotate::rotate;
pub use split::{split_each, split_range};
