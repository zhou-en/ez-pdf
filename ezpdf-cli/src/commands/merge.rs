use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, merge};
use lopdf::Document;

use crate::output::{maybe_progress, print_success, resolve_input, resolve_password};

#[derive(Args)]
pub struct MergeArgs {
    /// Input PDF files to merge (in order), or a single directory with --batch
    #[arg(required = true, num_args = 1..)]
    pub files: Vec<PathBuf>,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Password for encrypted input PDFs (applied to all inputs)
    #[arg(long)]
    pub password: Option<String>,

    /// Read password from a file (strips trailing whitespace)
    #[arg(long, value_name = "FILE")]
    pub password_file: Option<PathBuf>,

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

    // Decrypt any encrypted inputs; _tmps keeps the NamedTempFiles alive
    let pw = resolve_password(args.password.as_deref(), args.password_file.as_deref())?;
    let mut resolved: Vec<PathBuf> = Vec::with_capacity(inputs.len());
    let mut _tmps = Vec::new();
    for path in &inputs {
        let (resolved_path, tmp) = resolve_input(path, pw.as_deref())?;
        resolved.push(resolved_path);
        if let Some(t) = tmp {
            _tmps.push(t);
        }
    }

    let total_pages: u32 = resolved
        .iter()
        .filter_map(|p| Document::load(p).ok())
        .map(|d| d.get_pages().len() as u32)
        .sum();

    let pb = maybe_progress("merge", total_pages, args.quiet);

    let refs: Vec<&std::path::Path> = resolved.iter().map(|p| p.as_path()).collect();
    merge(&refs, &args.output)?;

    if let Some(pb) = pb {
        pb.finish_and_clear();
    }

    print_success(
        &format!("Merged {} files → {}", resolved.len(), args.output.display()),
        args.quiet,
    );
    Ok(())
}
