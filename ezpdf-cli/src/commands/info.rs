use std::path::PathBuf;

use clap::Args;
use ezpdf_core::info;

#[derive(Args)]
pub struct InfoArgs {
    /// Input PDF file
    pub file: PathBuf,

    /// Show dimensions for specific pages only (e.g. "1,3,5-7")
    #[arg(long)]
    pub pages: Option<String>,

    /// Output as JSON
    #[arg(long)]
    pub json: bool,
}

pub fn run(args: InfoArgs) -> anyhow::Result<()> {
    let pdf_info = info(&args.file)?;

    if args.json {
        println!("{}", serde_json::to_string_pretty(&pdf_info)?);
        return Ok(());
    }

    println!("File:  {}", args.file.display());
    println!("Pages: {}", pdf_info.page_count);
    println!();
    // Filter to requested pages if --pages flag provided
    let page_indices: Vec<usize> = if let Some(ref page_spec) = args.pages {
        ezpdf_core::page_range::parse(page_spec, pdf_info.page_count)
            .map_err(anyhow::Error::from)?
            .into_iter()
            .map(|p| (p - 1) as usize)
            .collect()
    } else {
        (0..pdf_info.dimensions.len()).collect()
    };

    println!("{:<6} {:>10} {:>10}", "Page", "Width pt", "Height pt");
    println!("{:-<6} {:->10} {:->10}", "", "", "");
    for i in &page_indices {
        if let Some((w, h)) = pdf_info.dimensions.get(*i) {
            let label = paper_size_label(*w, *h);
            println!("{:<6} {:>10.1} {:>10.1}  {}", i + 1, w, h, label);
        }
    }

    let fields = [
        ("Title", &pdf_info.title),
        ("Author", &pdf_info.author),
        ("Subject", &pdf_info.subject),
        ("Keywords", &pdf_info.keywords),
        ("Creator", &pdf_info.creator),
        ("Producer", &pdf_info.producer),
    ];
    let has_meta = fields.iter().any(|(_, v)| v.is_some());
    if has_meta {
        println!();
        for (label, value) in &fields {
            if let Some(v) = value {
                println!("{label:<10} {v}");
            }
        }
    }

    Ok(())
}

fn paper_size_label(w: f64, h: f64) -> &'static str {
    let matches = |a: f64, b: f64| (w - a).abs() < 2.0 && (h - b).abs() < 2.0;
    if matches(612.0, 792.0) {
        "(Letter)"
    } else if matches(595.0, 842.0) {
        "(A4)"
    } else if matches(612.0, 1008.0) {
        "(Legal)"
    } else {
        ""
    }
}
