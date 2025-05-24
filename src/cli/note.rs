use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use inkjet::Highlighter;
use tera::Tera;
use tracing::{info, instrument};

use crate::{
    djot::{DemoteHeadings, InkjetCode, KatexMath, ShowErrors},
    doc::Doc,
};

#[derive(Debug, clap::Args)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Build(cmd) => cmd.run(),
            Command::New(cmd) => cmd.run(),
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Build the notes.
    Build(BuildCommand),

    /// Create a new note.
    New(NewCommand),
}

/// Build the notes.
#[derive(Debug, clap::Args)]
pub struct BuildCommand {}

impl BuildCommand {
    pub fn run(self) -> Result<()> {
        let notes_dir: PathBuf = "notes/".into();

        if !notes_dir.exists() {
            info!("notes directory does not exist");
            return Ok(());
        }

        info!("building notes from: {:?}", notes_dir);

        for entry in std::fs::read_dir(notes_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map_or(false, |ext| ext == "dj") {
                self.build_note(&path)?;
            }
        }

        Ok(())
    }

    fn build_note(&self, path: &Path) -> Result<()> {
        info!("building note: {:?}", path);

        let content = std::fs::read_to_string(path)?;

        let doc = Doc::new(&content)?;

        let katex_opts = katex::Opts::builder()
            .macros(doc.header.math.macros.clone())
            .build()
            .unwrap();

        let highlighter = Highlighter::new();

        let parser = jotdown::Parser::new(doc.body);
        let parser = DemoteHeadings::new(parser, 1);
        let parser = KatexMath::new(parser, katex_opts);
        let parser = ShowErrors::new(parser);
        let parser = InkjetCode::new(parser, highlighter);
        let parser = ShowErrors::new(parser);

        let html = jotdown::html::render_to_string(parser);

        let tera = Tera::new("templates/**/*")?;
        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("body", &html);
        tera_ctx.insert("title", &doc.header.title);
        let rendered = tera.render("notes/note.html", &tera_ctx)?;

        let dist_dir: PathBuf = "dist/notes/".into();
        if !dist_dir.exists() {
            std::fs::create_dir_all(&dist_dir)?;
        }

        let path_with_html = path.with_extension("html");
        let output_file = path_with_html.file_name().unwrap();
        let output_path = dist_dir.join(output_file);

        info!("writing rendered HTML to: {:?}", output_path);
        std::fs::write(output_path, rendered)?;

        Ok(())
    }
}

/// Create a new note.
#[derive(Debug, clap::Args)]
pub struct NewCommand {
    name: String,

    /// Do not open the note in $EDITOR.
    #[clap(short = 'n')]
    no_editor: bool,
}

impl NewCommand {
    #[instrument(skip_all, err)]
    pub fn run(self) -> Result<()> {
        let notes_dir: PathBuf = "notes/".into();

        info!("notes directory: {:?}", notes_dir);

        if !notes_dir.exists() {
            info!("creating notes directory at: {:?}", notes_dir);
            std::fs::create_dir_all(&notes_dir)?;
        }

        let date = chrono::Local::now().format("%Y-%m-%d").to_string();

        let note_path = notes_dir.join(format!("{}-{}.dj", date, self.name));

        if !note_path.exists() {
            info!("creating note at: {:?}", note_path);
            let templates = Templates::new()?;
            let rendered = templates.new_note(&date)?;
            std::fs::write(&note_path, rendered)?;
            info!("note was created successfully!");
        } else {
            info!("note already exists at: {:?}", note_path);
        }

        if !self.no_editor {
            self.open_editor(&note_path)?;
        }

        Ok(())
    }

    fn open_editor(&self, path: &Path) -> Result<()> {
        let Ok(editor) = std::env::var("EDITOR") else {
            info!("EDITOR environment variable not set");
            return Ok(());
        };

        info!("opening note in editor: {}", editor);
        let status = std::process::Command::new(editor).arg(path).status()?;

        if !status.success() {
            bail!("editor exited with non-zero status: {}", status);
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Templates {
    tera: Tera,
}

impl Templates {
    pub fn new() -> Result<Self> {
        let mut tera = Tera::new("templates/**/*")?;
        // tera.add_template_file("notes/layout.html", None)?;
        Ok(Self { tera })
    }

    pub fn new_note(&self, date: &str) -> Result<String> {
        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("date", date);
        let rendered = self.tera.render("notes/new.dj", &tera_ctx)?;
        Ok(rendered)
    }

    pub fn note(&self, doc: &Doc, html: &str) -> Result<String> {
        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("meta", &doc.header);
        tera_ctx.insert("title", &doc.header.title);
        tera_ctx.insert("date", &doc.header.date);
        tera_ctx.insert("html", &html);
        let rendered = self.tera.render("notes/new.dj", &tera_ctx)?;
        Ok(rendered)
    }
}

impl Default for Templates {
    fn default() -> Self {
        todo!()
    }
}
