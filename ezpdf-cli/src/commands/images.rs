use std::path::PathBuf;

use clap::Args;
use ezpdf_core::extract_images;

#[derive(Args)]
pub struct ImagesArgs {
    /// Input PDF file
    pub file: PathBuf,

    /// Output directory for extracted images
    #[arg(short, long)]
    pub output: PathBuf,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: ImagesArgs) -> anyhow::Result<()> {
    let count = extract_images(&args.file, &args.output)?;

    if !args.quiet {
        if count == 1 {
            println!("Extracted 1 image → {}", args.output.display());
        } else {
            println!("Extracted {count} images → {}", args.output.display());
        }
    }

    Ok(())
}
