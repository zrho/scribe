use std::path::{Path, PathBuf};

use anyhow::{Result, bail};
use tera::Tera;
use tracing::{info, instrument};

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
            let rendered = self.render_note(&date)?;
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

    fn render_note(&self, date: &str) -> Result<String> {
        let tera = Tera::new("templates/**/*")?;
        let mut tera_ctx = tera::Context::new();
        tera_ctx.insert("date", date);
        let rendered = tera.render("new-note.dj", &tera_ctx)?;
        Ok(rendered)
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
