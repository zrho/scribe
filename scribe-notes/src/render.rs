use std::{fs, path::Path};

use crate::{header::Header, templates::Templates};
use anyhow::Result;
use inkjet::Highlighter;
use scribe_common::djot::{DemoteHeadings, InkjetCode, KatexMath, ShowErrors, parse_frontmatter};
use tracing::instrument;

#[instrument(skip(templates), name = "rendering note files")]
pub fn render_note_files(input_dir: &Path, output_dir: &Path, templates: &Templates) -> Result<()> {
    let pattern = input_dir.join("*.dj");
    let glob_pattern = pattern.to_string_lossy();

    // Create output directory if it doesn't exist.
    fs::create_dir_all(output_dir)?;

    for entry in glob::glob(&glob_pattern)? {
        let input_file = entry?;
        let rel_path = input_file.strip_prefix(input_dir)?;
        let mut output_path = output_dir.join(rel_path);
        output_path.set_extension("html");
        render_note_file(&input_file, &output_path, templates)?;
    }

    Ok(())
}

#[instrument(err, skip(templates))]
pub fn render_note_file(
    input_file: &Path,
    output_file: &Path,
    templates: &Templates,
) -> Result<()> {
    let source = fs::read_to_string(input_file)?;
    let html = render_note(&source, templates)?;
    fs::write(output_file, html)?;
    Ok(())
}

pub fn render_note(source: &str, templates: &Templates) -> Result<String> {
    let (header, body) = parse_frontmatter::<Header>(source)?;

    let katex_opts = katex::Opts::builder()
        .macros(header.math.macros.clone())
        .build()
        .unwrap();

    let highlighter = Highlighter::new();

    let parser = jotdown::Parser::new(body);
    let parser = DemoteHeadings::new(parser, 1);
    let parser = KatexMath::new(parser, katex_opts);
    let parser = ShowErrors::new(parser);
    let parser = InkjetCode::new(parser, highlighter);
    let parser = ShowErrors::new(parser);
    let body = jotdown::html::render_to_string(parser);

    let html = templates.render_note(&header, &body)?;
    Ok(html)
}
