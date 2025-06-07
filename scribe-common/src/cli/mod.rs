use anyhow::Result;

pub mod note;
pub mod render;

#[derive(Debug, clap::Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Command,
}

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.command {
            Command::Render(cmd) => cmd.run(),
            Command::Note(cmd) => cmd.run(),
        }
    }
}

#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Render documents.
    Render(render::Cli),
    /// Notes.
    Note(note::Cli),
}
