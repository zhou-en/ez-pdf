mod commands;
mod output;

use clap::{Parser, Subcommand};
use commands::bookmarks::BookmarksArgs;
use commands::completions::CompletionsArgs;
use commands::info::InfoArgs;
use commands::merge::MergeArgs;
use commands::meta::MetaArgs;
use commands::remove::RemoveArgs;
use commands::reorder::ReorderArgs;
use commands::rotate::RotateArgs;
use commands::split::SplitArgs;
use commands::watermark::WatermarkArgs;

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
    /// Manage PDF bookmarks (outline entries)
    Bookmarks(BookmarksArgs),
    /// Generate shell completion scripts
    Completions(CompletionsArgs),
    /// Show page count, dimensions, and metadata of a PDF
    Info(InfoArgs),
    /// Read or write PDF metadata fields
    Meta(MetaArgs),
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
    /// Stamp a diagonal text watermark onto pages
    Watermark(WatermarkArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Bookmarks(args) => commands::bookmarks::run(args),
        Commands::Completions(args) => commands::completions::run(args),
        Commands::Info(args) => commands::info::run(args),
        Commands::Meta(args) => commands::meta::run(args),
        Commands::Merge(args) => commands::merge::run(args),
        Commands::Remove(args) => commands::remove::run(args),
        Commands::Reorder(args) => commands::reorder::run(args),
        Commands::Rotate(args) => commands::rotate::run(args),
        Commands::Split(args) => commands::split::run(args),
        Commands::Watermark(args) => commands::watermark::run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
