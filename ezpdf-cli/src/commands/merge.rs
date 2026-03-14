use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, merge};
use lopdf::Document;

use crate::output::{maybe_progress, print_success};

#[derive(Args)]
pub struct MergeArgs {
    /// Input PDF files to merge (in order), or a single directory with --batch
    #[arg(required = true, num_args = 1..)]
    pub files: Vec<PathBuf>,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Merge all PDFs in input directory into one output file
    #[arg(long)]
    pub batch: bool,

    /// Suppress output
    #[arg(short, long, global = false)]
    pub quiet: bool,
}

pub fn run(args: MergeArgs) -> anyhow::Result<()> {
    let inputs: Vec<PathBuf> = if args.batch {
        let dir = args.files.first().ok_or_else(|| anyhow::anyhow!("No directory specified"))?;
        collect_pdf_inputs(dir)?
    } else {
        args.files.clone()
    };

    let total_pages: u32 = inputs
        .iter()
        .filter_map(|p| Document::load(p).ok())
        .map(|d| d.get_pages().len() as u32)
        .sum();

    let pb = maybe_progress("merge", total_pages, args.quiet);

    let refs: Vec<&std::path::Path> = inputs.iter().map(|p| p.as_path()).collect();
    merge(&refs, &args.output)?;

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    print_success(
        &format!("Merged {} files → {}", inputs.len(), args.output.display()),
        args.quiet,
    );
    Ok(())
}
