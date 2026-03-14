use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, rotate};

use crate::output::{print_success, run_batch_independent};

#[derive(Args)]
pub struct RotateArgs {
    /// Input PDF file (or directory when --batch is set)
    pub input: PathBuf,

    /// Rotation in degrees (90, 180, 270, or negative multiples of 90)
    pub degrees: i32,

    /// Pages to rotate (e.g. "1,3,5"). Omit to rotate all pages.
    #[arg(long)]
    pub pages: Option<String>,

    /// Output PDF file path (or directory when --batch is set)
    #[arg(short, long)]
    pub output: PathBuf,

    /// Process all PDFs in input directory, writing results to output directory
    #[arg(long)]
    pub batch: bool,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: RotateArgs) -> anyhow::Result<()> {
    if args.batch {
        let inputs = collect_pdf_inputs(&args.input)?;
        let degrees = args.degrees;
        let pages = args.pages.clone();
        run_batch_independent(
            inputs,
            &args.output,
            args.quiet,
            &format!("Rotated {degrees}°"),
            move |src, dst| rotate(src, degrees, pages.as_deref(), dst),
        )?;
    } else {
        rotate(&args.input, args.degrees, args.pages.as_deref(), &args.output)?;
        print_success(
            &format!("Rotated {}° → {}", args.degrees, args.output.display()),
            args.quiet,
        );
    }
    Ok(())
}
