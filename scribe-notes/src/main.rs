use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;
use tracing::{info, instrument};

use crate::{
    config::{NOTES_INPUT_DIR, NOTES_OUTPUT_DIR},
    render::render_note_files,
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
}

pub fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build {} => {
            info!("building notes...");
            build()?;
        }
        Commands::Watch {} => {
            println!("Watching for changes...");
            // Add watch logic here
        }
        Commands::Serve {} => {
            println!("Serving application...");
            // Add serve logic here
        }
        Commands::Clean {} => {
            println!("Cleaning build artifacts...");
            // Add clean logic here
        }
    }

    Ok(())
}

#[instrument(name = "build")]
fn build() -> Result<()> {
    let notes_input_dir: PathBuf = NOTES_INPUT_DIR.into();
    let notes_output_dir: PathBuf = NOTES_OUTPUT_DIR.into();
    let templates = Templates::new()?;
    render_note_files(&notes_input_dir, &notes_output_dir, &templates)?;
    Ok(())
}
