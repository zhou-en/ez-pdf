use std::path::PathBuf;
use std::process::Command as StdCommand;

use clap::Args;
use ezpdf_core::optimize;

#[derive(Args)]
pub struct OptimizeArgs {
    /// Input PDF file
    pub file: PathBuf,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Attempt linearization via qpdf (skipped with warning if qpdf is not installed)
    #[arg(long)]
    pub linearize: bool,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: OptimizeArgs) -> anyhow::Result<()> {
    let stats = optimize(&args.file, &args.output)?;

    if args.linearize {
        let status = StdCommand::new("qpdf")
            .args([
                "--linearize",
                args.output.to_str().unwrap_or(""),
                args.output.to_str().unwrap_or(""),
            ])
            .status();
        if status.is_err() {
            eprintln!("Warning: qpdf not found — skipping linearization");
        }
    }

    if !args.quiet {
        println!(
            "Optimized → {} ({} object{} removed, {} bytes saved)",
            args.output.display(),
            stats.objects_removed,
            if stats.objects_removed == 1 { "" } else { "s" },
            stats.bytes_saved,
        );
    }

    Ok(())
}
