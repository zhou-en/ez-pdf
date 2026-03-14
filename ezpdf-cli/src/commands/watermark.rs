use std::path::PathBuf;

use clap::Args;
use ezpdf_core::{watermark, watermark_mod::WatermarkOptions};

use crate::output::print_success;

#[derive(Args)]
pub struct WatermarkArgs {
    /// Input PDF file
    pub input: PathBuf,

    /// Watermark text to stamp on each page
    pub text: String,

    /// Output PDF file path
    #[arg(short, long)]
    pub output: PathBuf,

    /// Opacity (0.0 = transparent, 1.0 = opaque)
    #[arg(long, default_value = "0.3")]
    pub opacity: f32,

    /// Font size in points
    #[arg(long, default_value = "48")]
    pub font_size: f32,

    /// Text color as R,G,B (each 0.0–1.0, e.g. "0.5,0.5,0.5")
    #[arg(long, default_value = "0.5,0.5,0.5")]
    pub color: String,

    /// Pages to watermark (e.g. "1,3,5"). Omit to watermark all pages.
    #[arg(long)]
    pub pages: Option<String>,

    /// Suppress output
    #[arg(short, long)]
    pub quiet: bool,
}

pub fn run(args: WatermarkArgs) -> anyhow::Result<()> {
    let color_rgb = parse_color(&args.color)?;

    let opts = WatermarkOptions {
        opacity: args.opacity,
        color_rgb,
        font_size: args.font_size,
        pages: args.pages,
    };

    watermark(&args.input, &args.text, opts, &args.output)?;
    print_success(
        &format!("Watermarked '{}' → {}", args.text, args.output.display()),
        args.quiet,
    );
    Ok(())
}

fn parse_color(s: &str) -> anyhow::Result<(f32, f32, f32)> {
    let parts: Vec<&str> = s.splitn(3, ',').collect();
    if parts.len() != 3 {
        anyhow::bail!("--color must be R,G,B (e.g. '0.5,0.5,0.5')");
    }
    let r = parts[0].trim().parse::<f32>()?;
    let g = parts[1].trim().parse::<f32>()?;
    let b = parts[2].trim().parse::<f32>()?;
    Ok((r, g, b))
}
