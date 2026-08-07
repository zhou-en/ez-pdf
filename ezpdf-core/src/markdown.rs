use std::path::Path;

use pdf_inspector::{MarkdownOptions as InspectorOptions, PdfError, PdfOptions};

use crate::error::EzPdfError;

/// Options controlling Markdown conversion.
#[derive(Debug, Clone)]
pub struct MarkdownOptions {
    /// Page selection in `1-5,7` syntax. `None` converts the whole document.
    pub pages: Option<String>,
    /// Emit a `---` separator between pages.
    pub page_breaks: bool,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            pages: None,
            page_breaks: true,
        }
    }
}

/// Converts a PDF to Markdown and returns it as a string.
///
/// This is a read-only extractor: it never writes a PDF, so the lossless
/// guarantee that governs the editing operations does not apply here.
pub fn to_markdown(input: &Path, options: &MarkdownOptions) -> Result<String, EzPdfError> {
    let bytes = std::fs::read(input)?;

    // page_range::parse owns range validation and the out-of-range error, and
    // needs the true page count to validate against.
    let selected = match &options.pages {
        Some(spec) => {
            let total = crate::info::page_count(input)?;
            Some(crate::page_range::parse(spec, total)?)
        }
        None => None,
    };

    // Page markers are what we split on, so ask for them whenever the caller
    // wants separators.
    let mut inspector_options = PdfOptions::new().markdown(InspectorOptions {
        include_page_numbers: options.page_breaks,
        ..Default::default()
    });
    if let Some(pages) = &selected {
        inspector_options = inspector_options.pages(pages.iter().copied());
    }

    let result = pdf_inspector::process_pdf_mem_with_options(&bytes, inspector_options)
        .map_err(map_error)?;

    let needs_ocr: Vec<u32> = match &selected {
        // A filtered run reports pages it never extracted as needing OCR, so
        // take the signal from an unfiltered detection pass and keep only the
        // pages the caller actually asked for.
        Some(pages) => {
            let detected = pdf_inspector::detect_pdf_mem(&bytes).map_err(map_error)?;
            detected
                .pages_needing_ocr
                .into_iter()
                .filter(|page| pages.contains(page))
                .collect()
        }
        None => result.pages_needing_ocr,
    };

    if !needs_ocr.is_empty() {
        return Err(EzPdfError::NoTextLayer { pages: needs_ocr });
    }

    let markdown = result.markdown.ok_or_else(|| EzPdfError::NoTextLayer {
        pages: (1..=result.page_count).collect(),
    })?;

    Ok(if options.page_breaks {
        convert_page_markers(&markdown)
    } else {
        markdown
    })
}

/// Converts a PDF to Markdown and writes it to `output`.
pub fn markdown(input: &Path, options: &MarkdownOptions, output: &Path) -> Result<(), EzPdfError> {
    let content = to_markdown(input, options)?;
    std::fs::write(output, content)?;
    Ok(())
}

fn map_error(error: PdfError) -> EzPdfError {
    match error {
        PdfError::Io(e) => EzPdfError::Io(e),
        PdfError::Encrypted => EzPdfError::EncryptedPdf,
        other => EzPdfError::Pdf(other.to_string()),
    }
}

/// Rewrites `<!-- Page N -->` markers as `---` separators.
///
/// The marker before the first page is dropped rather than converted: a
/// leading `---` is YAML frontmatter to Obsidian, Jekyll and Hugo.
fn convert_page_markers(markdown: &str) -> String {
    let mut out = String::with_capacity(markdown.len());
    let mut seen_page = false;

    for line in markdown.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("<!-- Page ") && trimmed.ends_with("-->") {
            if seen_page {
                out.push_str("\n---\n\n");
            }
            seen_page = true;
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }

    out.trim_start().to_string()
}
