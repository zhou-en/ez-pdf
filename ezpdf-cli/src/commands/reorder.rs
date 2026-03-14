use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, reorder};

use crate::output::{print_success, run_batch_independent};

#[derive(Args)]
pub struct ReorderArgs {
    /// Input PDF file (or directory when --batch is set)
    pub input: PathBuf,

    /// New page order as comma-separated 1-indexed page numbers (e.g. "3,1,2")
    pub order: String,

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

pub fn run(args: ReorderArgs) -> anyhow::Result<()> {
    if args.batch {
        let inputs = collect_pdf_inputs(&args.input)?;
        let order = args.order.clone();
        run_batch_independent(
            inputs,
            &args.output,
            args.quiet,
            "Reordered",
            move |src, dst| reorder(src, &order, dst),
        )?;
    } else {
        reorder(&args.input, &args.order, &args.output)?;
        print_success(
            &format!("Reordered → {}", args.output.display()),
            args.quiet,
        );
    }
    Ok(())
}
