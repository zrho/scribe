use clap_stdin::FileOrStdin;

/// Render documents.
#[derive(Debug, clap::Args)]
pub struct RenderCommand {
    /// Input to render (file or stdin).
    #[clap(value_name = "INPUT")]
    pub input: FileOrStdin,

    /// The output format.
    #[clap(long = "to")]
    pub output_format: OutputFormat,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum OutputFormat {
    Html,
    Latex,
}
