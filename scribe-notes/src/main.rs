use anyhow::Result;
use clap::Parser as _;

pub mod config;

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
            println!("Building project...");
            // Add build logic here
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
