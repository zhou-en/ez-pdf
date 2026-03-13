mod commands;
mod output;

use clap::{Parser, Subcommand};
use commands::completions::CompletionsArgs;
use commands::merge::MergeArgs;
use commands::remove::RemoveArgs;
use commands::reorder::ReorderArgs;
use commands::rotate::RotateArgs;
use commands::split::SplitArgs;

#[derive(Parser)]
#[command(
    name = "ezpdf",
    version,
    about = "Fast lossless PDF manipulation",
    long_about = "ezpdf — fast, lossless PDF manipulation from the command line.\n\nExamples:\n  ezpdf merge a.pdf b.pdf -o combined.pdf\n  ezpdf remove input.pdf 3,5 -o output.pdf\n  ezpdf rotate input.pdf 90 -o rotated.pdf\n  ezpdf split input.pdf 1-5 -o part.pdf\n  ezpdf reorder input.pdf 3,1,2 -o reordered.pdf"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate shell completion scripts
    Completions(CompletionsArgs),
    /// Merge two or more PDFs into one
    Merge(MergeArgs),
    /// Remove specific pages from a PDF
    Remove(RemoveArgs),
    /// Reorder pages by specifying a new page order
    Reorder(ReorderArgs),
    /// Rotate all or specific pages
    Rotate(RotateArgs),
    /// Extract a page range or burst into individual pages
    Split(SplitArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Completions(args) => commands::completions::run(args),
        Commands::Merge(args) => commands::merge::run(args),
        Commands::Remove(args) => commands::remove::run(args),
        Commands::Reorder(args) => commands::reorder::run(args),
        Commands::Rotate(args) => commands::rotate::run(args),
        Commands::Split(args) => commands::split::run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
