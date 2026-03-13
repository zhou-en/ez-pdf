use std::path::PathBuf;

use clap::Args;
use ezpdf_core::remove;
use lopdf::Document;

use crate::output::{maybe_progress, print_success};

#[derive(Args)]
pub struct RemoveArgs {
    /// Input PDF file
    pub input: PathBuf,

    /// Pages to remove (e.g. "3,5,7-9")
    pub pages: String,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: RemoveArgs) -> anyhow::Result<()> {
    let page_count = Document::load(&args.input)
        .map(|d| d.get_pages().len() as u32)
        .unwrap_or(0);
    let pb = maybe_progress("remove", page_count, args.quiet);

    remove(&args.input, &args.pages, &args.output)?;

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }
    print_success(
        &format!("Removed pages {} → {}", args.pages, args.output.display()),
        args.quiet,
    );
    Ok(())
}
