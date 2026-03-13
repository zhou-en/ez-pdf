use std::path::PathBuf;

use clap::Args;
use ezpdf_core::remove;

use crate::output::print_success;

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
    remove(&args.input, &args.pages, &args.output)?;
    print_success(
        &format!("Removed pages {} → {}", args.pages, args.output.display()),
        args.quiet,
    );
    Ok(())
}
