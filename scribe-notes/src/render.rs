use std::{fs, path::Path};

use crate::{
    header::Header,
    templates::{NoteData, Templates},
};
use anyhow::Result;
use inkjet::Highlighter;
use scribe_common::djot::{DemoteHeadings, InkjetCode, KatexMath, ShowErrors, parse_frontmatter};
use tracing::{Level, info, instrument};

pub fn render_index_file(input_dir: &Path, output_dir: &Path, templates: &Templates) -> Result<()> {
    let pattern = input_dir.join("*.dj");
    let glob_pattern = pattern.to_string_lossy();

    let mut headers = Vec::new();

    for entry in glob::glob(&glob_pattern)? {
        let input_file = entry?;
        let source = fs::read_to_string(&input_file)?;
        let (header, _) = parse_frontmatter::<Header>(&source)?;
        let link = format!("/notes/{}.html", input_file.file_stem().unwrap().display());
        headers.push(NoteData { header, link });
    }

    let rendered = templates.render_index(&headers)?;

    let output_file = output_dir.join("index.html");
    fs::write(output_file, rendered)?;

    Ok(())
}

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

#[instrument(err, skip(templates, output_file))]
pub fn render_note_file(
    input_file: &Path,
    output_file: &Path,
    templates: &Templates,
) -> Result<()> {
    info!("rendering note...");
    let source = fs::read_to_string(input_file)?;
    let html = render_note(&source, templates)?;
    fs::write(output_file, html)?;
    Ok(())
}

pub fn render_note(source: &str, templates: &Templates) -> Result<String> {
    let (header, body) = parse_frontmatter::<Header>(source)?;

    let katex_opts = katex::Opts::builder()
        .macros(header.math.macros.clone())
        .output_type(katex::OutputType::Html)
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

pub fn copy_static_assets(assets_dir: &Path, dist_dir: &Path) -> Result<()> {
    if !assets_dir.exists() {
        return Ok(());
    }

    fs::create_dir_all(dist_dir)?;
    let glob_pattern = format!("{}/**/*", assets_dir.display());

    for entry in glob::glob(&glob_pattern)? {
        let path = entry?;

        if path.is_file() {
            let rel_path = path.strip_prefix(assets_dir)?;
            let dest_path = dist_dir.join(rel_path);

            // Create parent directories if they don't exist
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::copy(&path, &dest_path)?;
            info!("Copied asset: {:?}", rel_path);
        }
    }

    Ok(())
}
