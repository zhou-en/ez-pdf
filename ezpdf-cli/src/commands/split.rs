use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{split_each, split_range};

use crate::output::print_success;

#[derive(Args)]
pub struct SplitArgs {
    /// Input PDF file
    pub input: PathBuf,

    /// Page range to extract (e.g. "1-5,7"). Omit with --each to burst all pages.
    pub range: Option<String>,

    /// Burst into individual pages (one file per page)
    #[arg(long)]
    pub each: bool,

    /// Output path: file for range mode, directory for --each mode
    #[arg(short, long)]
    pub output: PathBuf,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: SplitArgs) -> anyhow::Result<()> {
    if args.each {
        split_each(&args.input, &args.output)?;
        print_success(
            &format!("Split into individual pages → {}", args.output.display()),
            args.quiet,
        );
    } else {
        let range = args.range.as_deref().ok_or_else(|| {
            anyhow::anyhow!("provide a page range (e.g. '1-3') or use --each to burst all pages")
        })?;
        split_range(&args.input, range, &args.output)?;
        print_success(
            &format!("Split pages {range} → {}", args.output.display()),
            args.quiet,
        );
    }
    Ok(())
}
