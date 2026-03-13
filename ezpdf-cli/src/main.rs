mod commands;
mod output;

use clap::{Parser, Subcommand};
use commands::merge::MergeArgs;
use commands::remove::RemoveArgs;

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
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Merge(args) => commands::merge::run(args),
        Commands::Remove(args) => commands::remove::run(args),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
