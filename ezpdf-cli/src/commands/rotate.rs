use std::path::PathBuf;

use clap::Args;
use ezpdf_core::rotate;

use crate::output::print_success;

#[derive(Args)]
pub struct RotateArgs {
    /// Input PDF file
    pub input: PathBuf,

    /// Rotation in degrees (90, 180, 270, or negative multiples of 90)
    pub degrees: i32,

    /// Pages to rotate (e.g. "1,3,5"). Omit to rotate all pages.
    #[arg(long)]
    pub pages: Option<String>,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: RotateArgs) -> anyhow::Result<()> {
    rotate(
        &args.input,
        args.degrees,
        args.pages.as_deref(),
        &args.output,
    )?;
    print_success(
        &format!("Rotated {}° → {}", args.degrees, args.output.display()),
        args.quiet,
    );
    Ok(())
}
