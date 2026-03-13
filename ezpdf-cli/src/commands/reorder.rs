use std::path::PathBuf;

use clap::Args;
use ezpdf_core::reorder;

use crate::output::print_success;

#[derive(Args)]
pub struct ReorderArgs {
    /// Input PDF file
    pub input: PathBuf,

    /// New page order as comma-separated 1-indexed page numbers (e.g. "3,1,2")
    pub order: String,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: ReorderArgs) -> anyhow::Result<()> {
    reorder(&args.input, &args.order, &args.output)?;
    print_success(
        &format!("Reordered → {}", args.output.display()),
        args.quiet,
    );
    Ok(())
}
