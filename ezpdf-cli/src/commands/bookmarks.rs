use std::path::PathBuf;

use clap::{Args, Subcommand};
use ezpdf_core::{add_bookmark, list_bookmarks};

use crate::output::print_success;

#[derive(Args)]
pub struct BookmarksArgs {
    #[command(subcommand)]
    pub command: BookmarksCommands,
}

#[derive(Subcommand)]
pub enum BookmarksCommands {
    /// List all bookmarks in a PDF
    List(ListArgs),
    /// Add a bookmark to a PDF
    Add(AddArgs),
}

#[derive(Args)]
pub struct ListArgs {
    /// Input PDF file
    pub file: PathBuf,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct AddArgs {
    /// Input PDF file
    pub file: PathBuf,

    /// Bookmark title
    #[arg(long)]
    pub title: String,

    /// Target page number (1-based)
    #[arg(long)]
    pub page: u32,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: BookmarksArgs) -> anyhow::Result<()> {
    match args.command {
        BookmarksCommands::List(a) => run_list(a),
        BookmarksCommands::Add(a) => run_add(a),
    }
}

fn run_list(args: ListArgs) -> anyhow::Result<()> {
    let bookmarks = list_bookmarks(&args.file)?;

    if args.json {
        let items: Vec<serde_json::Value> = bookmarks
            .iter()
            .map(|b| {
                serde_json::json!({
                    "title": b.title,
                    "page": b.page,
                    "level": b.level,
                })
            })
            .collect();
        println!("{}", serde_json::to_string_pretty(&items)?);
        return Ok(());
    }

    for b in &bookmarks {
        let indent = "  ".repeat(b.level as usize);
        println!("{}{} (page {})", indent, b.title, b.page);
    }

    Ok(())
}

fn run_add(args: AddArgs) -> anyhow::Result<()> {
    add_bookmark(&args.file, &args.title, args.page, &args.output)?;
    print_success(
        &format!(
            "Added bookmark \"{}\" → {}",
            args.title,
            args.output.display()
        ),
        args.quiet,
    );
    Ok(())
}
