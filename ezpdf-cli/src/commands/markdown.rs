use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{batch::collect_pdf_inputs, markdown, MarkdownOptions};

use crate::output::{print_success, resolve_input, resolve_password};

#[derive(Args)]
pub struct MarkdownArgs {
    /// Input PDF file (or directory when --batch is set)
    pub input: PathBuf,

    /// Output Markdown file path (or directory when --batch is set)
    #[arg(short, long)]
    pub output: PathBuf,

    /// Pages to convert (e.g. "1-5,7"). Omit to convert the whole document.
    #[arg(long)]
    pub pages: Option<String>,

    /// Omit the `---` separators between pages
    #[arg(long)]
    pub no_page_breaks: bool,

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

pub fn run(args: MarkdownArgs) -> anyhow::Result<()> {
    let options = MarkdownOptions {
        pages: args.pages.clone(),
        page_breaks: !args.no_page_breaks,
    };

    if args.batch {
        // run_batch_independent writes to <out_dir>/<original filename>, which
        // would emit Markdown into a .pdf file — so map the extension here.
        let inputs = collect_pdf_inputs(&args.input)?;
        std::fs::create_dir_all(&args.output)?;
        for input in &inputs {
            let stem = input.file_stem().ok_or_else(|| {
                anyhow::anyhow!("cannot determine filename for {}", input.display())
            })?;
            markdown(
                input,
                &options,
                &args.output.join(stem).with_extension("md"),
            )?;
        }
        print_success(
            &format!(
                "Converted to Markdown — {} files → {}",
                inputs.len(),
                args.output.display()
            ),
            args.quiet,
        );
    } else {
        let pw = resolve_password(args.password.as_deref(), args.password_file.as_deref())?;
        let (input, _tmp) = resolve_input(&args.input, pw.as_deref())?;
        markdown(&input, &options, &args.output)?;
        print_success(
            &format!("Converted to Markdown → {}", args.output.display()),
            args.quiet,
        );
    }
    Ok(())
}
