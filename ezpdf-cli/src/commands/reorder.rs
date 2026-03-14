use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, reorder};

use crate::output::{print_success, resolve_input, resolve_password, run_batch_independent};

#[derive(Args)]
pub struct ReorderArgs {
    /// Input PDF file (or directory when --batch is set)
    pub input: PathBuf,

    /// New page order as comma-separated 1-indexed page numbers (e.g. "3,1,2")
    pub order: String,

    /// Output PDF file path (or directory when --batch is set)
    #[arg(short, long)]
    pub output: PathBuf,

    /// Password for encrypted input PDF
    #[arg(long)]
    pub password: Option<String>,

    /// Read password from a file (strips trailing whitespace)
    #[arg(long, value_name = "FILE")]
    pub password_file: Option<PathBuf>,

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
        let pw = resolve_password(args.password.as_deref(), args.password_file.as_deref())?;
        let (input, _tmp) = resolve_input(&args.input, pw.as_deref())?;
        reorder(&input, &args.order, &args.output)?;
        print_success(
            &format!("Reordered → {}", args.output.display()),
            args.quiet,
        );
    }
    Ok(())
}
