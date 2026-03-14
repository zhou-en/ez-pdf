use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, split_each, split_range};
use lopdf::Document;

use crate::output::{maybe_progress, print_success, resolve_input, resolve_password};

#[derive(Args)]
pub struct SplitArgs {
    /// Input PDF file (or directory when --batch is set)
    pub input: PathBuf,

    /// Page range to extract (e.g. "1-5,7"). Omit with --each to burst all pages.
    pub range: Option<String>,

    /// Burst into individual pages (one file per page)
    #[arg(long)]
    pub each: bool,

    /// Output path: file for range mode, directory for --each or --batch mode
    #[arg(short, long)]
    pub output: PathBuf,

    /// Password for encrypted input PDF
    #[arg(long)]
    pub password: Option<String>,

    /// Read password from a file (strips trailing whitespace)
    #[arg(long, value_name = "FILE")]
    pub password_file: Option<PathBuf>,

    /// Process all PDFs in input directory, splitting each into its own subdirectory
    #[arg(long)]
    pub batch: bool,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: SplitArgs) -> anyhow::Result<()> {
    if args.batch {
        let inputs = collect_pdf_inputs(&args.input)?;
        std::fs::create_dir_all(&args.output)?;
        for path in &inputs {
            let stem = path.file_stem().unwrap().to_string_lossy();
            let sub_dir = args.output.join(stem.as_ref());
            std::fs::create_dir_all(&sub_dir)?;
            split_each(path, &sub_dir)?;
        }
        print_success(
            &format!("Split {} files → {}", inputs.len(), args.output.display()),
            args.quiet,
        );
        return Ok(());
    }

    let pw = resolve_password(args.password.as_deref(), args.password_file.as_deref())?;
    let (input, _tmp) = resolve_input(&args.input, pw.as_deref())?;
    let page_count = Document::load(&input)
        .map(|d| d.get_pages().len() as u32)
        .unwrap_or(0);

    if args.each {
        let pb = maybe_progress("split-each", page_count, args.quiet);
        split_each(&input, &args.output)?;
        if let Some(pb) = pb {
            pb.finish_and_clear();
        }
        print_success(
            &format!("Split into individual pages → {}", args.output.display()),
            args.quiet,
        );
    } else {
        let range = args.range.as_deref().ok_or_else(|| {
            anyhow::anyhow!("provide a page range (e.g. '1-3') or use --each to burst all pages")
        })?;
        split_range(&input, range, &args.output)?;
        print_success(
            &format!("Split pages {range} → {}", args.output.display()),
            args.quiet,
        );
    }
    Ok(())
}
