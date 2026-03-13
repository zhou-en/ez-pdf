mod commands;
mod output;

use clap::{Parser, Subcommand};
use commands::merge::MergeArgs;
use commands::remove::RemoveArgs;
use commands::rotate::RotateArgs;
use commands::split::SplitArgs;

#[derive(Parser)]
#[command(name = "ezpdf", version, about = "Fast lossless PDF manipulation")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Merge two or more PDFs into one
    Merge(MergeArgs),
    /// Remove specific pages from a PDF
    Remove(RemoveArgs),
    /// Rotate all or specific pages
    Rotate(RotateArgs),
    /// Extract a page range or burst into individual pages
    Split(SplitArgs),
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Merge(args) => commands::merge::run(args),
        Commands::Remove(args) => commands::remove::run(args),
        Commands::Rotate(args) => commands::rotate::run(args),
        Commands::Split(args) => commands::split::run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
