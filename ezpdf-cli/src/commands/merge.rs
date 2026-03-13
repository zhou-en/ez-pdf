use std::path::PathBuf;

use clap::Args;
use ezpdf_core::merge;

use crate::output::print_success;

#[derive(Args)]
pub struct MergeArgs {
    /// Input PDF files to merge (in order)
    #[arg(required = true, num_args = 1..)]
    pub files: Vec<PathBuf>,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Suppress output
    #[arg(short, long, global = false)]
    pub quiet: bool,
}

pub fn run(args: MergeArgs) -> anyhow::Result<()> {
    let inputs: Vec<&std::path::Path> = args.files.iter().map(|p| p.as_path()).collect();
    merge(&inputs, &args.output)?;
    print_success(
        &format!(
            "Merged {} files → {}",
            args.files.len(),
            args.output.display()
        ),
        args.quiet,
    );
    Ok(())
}
