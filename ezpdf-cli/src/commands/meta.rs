use std::path::PathBuf;

use clap::{Args, Subcommand};
use ezpdf_core::{get_metadata, metadata::MetadataUpdate, set_metadata};

use crate::output::print_success;

#[derive(Args)]
pub struct MetaArgs {
    #[command(subcommand)]
    pub command: MetaCommands,
}

#[derive(Subcommand)]
pub enum MetaCommands {
    /// Read and display PDF metadata fields
    Get(GetArgs),
    /// Write PDF metadata fields
    Set(SetArgs),
}

#[derive(Args)]
pub struct GetArgs {
    /// Input PDF file
    pub file: PathBuf,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

#[derive(Args)]
pub struct SetArgs {
    /// Input PDF file
    pub file: PathBuf,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Document title
    #[arg(long)]
    pub title: Option<String>,

    /// Document author
    #[arg(long)]
    pub author: Option<String>,

    /// Document subject
    #[arg(long)]
    pub subject: Option<String>,

    /// Document keywords
    #[arg(long)]
    pub keywords: Option<String>,

    /// Creating application
    #[arg(long)]
    pub creator: Option<String>,

    /// Producing application
    #[arg(long)]
    pub producer: Option<String>,

    /// Clear all existing metadata fields before applying updates
    #[arg(long)]
    pub clear_all: bool,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: MetaArgs) -> anyhow::Result<()> {
    match args.command {
        MetaCommands::Get(a) => run_get(a),
        MetaCommands::Set(a) => run_set(a),
    }
}

fn run_get(args: GetArgs) -> anyhow::Result<()> {
    let meta = get_metadata(&args.file)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&meta)?);
        return Ok(());
    }

    let fields = [
        ("Title", &meta.title),
        ("Author", &meta.author),
        ("Subject", &meta.subject),
        ("Keywords", &meta.keywords),
        ("Creator", &meta.creator),
        ("Producer", &meta.producer),
    ];

    for (label, value) in &fields {
        if let Some(v) = value {
            println!("{label:<10} {v}");
        }
    }

    Ok(())
}

fn run_set(args: SetArgs) -> anyhow::Result<()> {
    let updates = MetadataUpdate {
        title: args.title,
        author: args.author,
        subject: args.subject,
        keywords: args.keywords,
        creator: args.creator,
        producer: args.producer,
        clear_all: args.clear_all,
    };

    set_metadata(&args.file, updates, &args.output)?;
    print_success(
        &format!("Updated metadata → {}", args.output.display()),
        args.quiet,
    );
    Ok(())
}
