use std::{
    path::{Path, PathBuf},
    sync::mpsc,
};

use anyhow::{Result, bail};
use clap::Parser as _;
use tracing::{error, info, instrument, trace};

use crate::{
    config::{ASSETS_DIR, DIST_DIR, NOTES_INPUT_DIR, NOTES_OUTPUT_DIR, TEMPLATES_DIR},
    render::{copy_static_assets, render_index_file, render_note_files},
    templates::Templates,
};

pub mod config;
pub mod header;
pub mod render;
pub mod templates;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(clap::Subcommand)]
pub enum Commands {
    /// Build the notes
    Build {},
    /// Watch for changes and rebuild
    Watch {},
    /// Watch and serve notes via http server
    Serve {},
    /// Clean build artifacts
    Clean {},
    /// Creare a new note.
    New(NewCommand),
}

/// Create a new note.
#[derive(Debug, clap::Args)]
pub struct NewCommand {
    /// The name of the node to create.
    name: String,

    /// Open the newly created note in the $EDITOR.
    #[clap(short = 'e')]
    edit: bool,
}

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Build {} => {
            build()?;
        }
        Commands::Watch {} => {
            watch()?;
        }
        Commands::Serve {} => {
            watch()?;
            serve().await?;
        }
        Commands::Clean {} => {
            println!("Cleaning build artifacts...");
            // Add clean logic here
        }
        Commands::New(cmd) => {
            new_note(cmd)?;
        }
    }

    Ok(())
}

#[instrument(name = "build")]
fn build() -> Result<()> {
    info!("building notes...");
    let notes_input_dir: PathBuf = NOTES_INPUT_DIR.into();
    let notes_output_dir: PathBuf = NOTES_OUTPUT_DIR.into();
    let dist_dir: PathBuf = DIST_DIR.into();
    let assets_dir: PathBuf = ASSETS_DIR.into();
    let templates = Templates::new()?;

    render_index_file(&notes_input_dir, &notes_output_dir, &templates)?;
    render_note_files(&notes_input_dir, &notes_output_dir, &templates)?;
    copy_static_assets(&assets_dir, &dist_dir)?;
    Ok(())
}

fn watch() -> Result<()> {
    use notify::Event;
    use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
    use std::time::Duration;

    std::thread::spawn(|| -> Result<()> {
        let (tx, rx) = mpsc::channel();
        let _ = tx.send(Ok(Event::default()));

        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;
        // Watch the notes directory for changes
        watcher.watch(NOTES_INPUT_DIR.as_ref(), RecursiveMode::Recursive)?;
        watcher.watch(ASSETS_DIR.as_ref(), RecursiveMode::Recursive)?;
        watcher.watch(TEMPLATES_DIR.as_ref(), RecursiveMode::Recursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    trace!("watch event: {:?}", event);
                    let result = build();

                    if let Err(error) = result {
                        error!("Error while building: {:?}", error);
                    }
                }
                Err(e) => {
                    error!("Watch error: {:?}", e);
                }
            }

            std::thread::sleep(Duration::from_millis(100));
        }

        info!("stopped watching");
        Ok(())
    });

    Ok(())
}

async fn serve() -> Result<()> {
    use axum::Router;
    use tower_http::services::ServeDir;

    info!("Starting HTTP server...");
    let app = Router::new().fallback_service(ServeDir::new(DIST_DIR));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

fn new_note(cmd: NewCommand) -> Result<()> {
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let note_dir = Path::new(NOTES_INPUT_DIR);
    let note_path = note_dir.join(format!("{}-{}.dj", date, cmd.name));

    if !note_path.exists() {
        info!("creating note at: {}", note_path.display());
        std::fs::write(&note_path, "")?;
        info!("note created");
    } else {
        info!("note already exists at: {}", note_path.display());
    }

    if cmd.edit {
        info!("opening note in editor");
        open_editor(&note_path)?;
    }

    Ok(())
}

fn open_editor(path: &Path) -> Result<()> {
    let Ok(editor) = std::env::var("EDITOR") else {
        info!("EDITOR environment variable not set");
        return Ok(());
    };

    let status = std::process::Command::new(editor).arg(path).status()?;

    if !status.success() {
        bail!("editor exited with non-zero status: {}", status);
    }

    Ok(())
}
