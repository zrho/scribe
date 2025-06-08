use std::path::PathBuf;

use anyhow::Result;
use clap::Parser as _;
use tracing::{info, instrument};

use crate::{
    config::{ASSETS_DIR, DIST_DIR, NOTES_INPUT_DIR, NOTES_OUTPUT_DIR},
    render::{copy_static_assets, render_note_files},
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

#[tokio::main]
pub async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Build {} => {
            build()?;
        }
        Commands::Watch {} => {
            println!("Watching for changes...");
            // Add watch logic here
        }
        Commands::Serve {} => {
            build()?;
            serve().await?;
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
    info!("building notes...");
    let notes_input_dir: PathBuf = NOTES_INPUT_DIR.into();
    let notes_output_dir: PathBuf = NOTES_OUTPUT_DIR.into();
    let dist_dir: PathBuf = DIST_DIR.into();
    let assets_dir: PathBuf = ASSETS_DIR.into();
    let templates = Templates::new()?;

    render_note_files(&notes_input_dir, &notes_output_dir, &templates)?;
    copy_static_assets(&assets_dir, &dist_dir)?;
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
