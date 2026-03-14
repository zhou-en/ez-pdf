use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, remove};
use lopdf::Document;

use crate::output::{maybe_progress, print_success, resolve_input, resolve_password, run_batch_independent};

#[derive(Args)]
pub struct RemoveArgs {
    /// Input PDF file (or directory when --batch is set)
    pub input: PathBuf,

    /// Pages to remove (e.g. "3,5,7-9")
    pub pages: String,

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

pub fn run(args: RemoveArgs) -> anyhow::Result<()> {
    if args.batch {
        let inputs = collect_pdf_inputs(&args.input)?;
        let pages = args.pages.clone();
        run_batch_independent(
            inputs,
            &args.output,
            args.quiet,
            &format!("Removed pages {}", args.pages),
            move |src, dst| remove(src, &pages, dst),
        )?;
    } else {
        let pw = resolve_password(args.password.as_deref(), args.password_file.as_deref())?;
        let (input, _tmp) = resolve_input(&args.input, pw.as_deref())?;
        let page_count = Document::load(&input)
            .map(|d| d.get_pages().len() as u32)
            .unwrap_or(0);
        let pb = maybe_progress("remove", page_count, args.quiet);
        remove(&input, &args.pages, &args.output)?;
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        print_success(
            &format!("Removed pages {} → {}", args.pages, args.output.display()),
            args.quiet,
        );
    }
    Ok(())
}
