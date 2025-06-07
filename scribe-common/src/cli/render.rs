use anyhow::{Result, bail};
use clap_stdin::FileOrStdin;
use inkjet::Highlighter;

use crate::{
    djot::{DemoteHeadings, InkjetCode, KatexMath, ShowErrors},
    doc::Doc,
};

/// Render documents.
#[derive(Debug, clap::Args)]
pub struct Cli {
    /// Djot input to render (file or stdin).
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

impl Cli {
    pub fn run(self) -> Result<()> {
        match self.output_format {
            OutputFormat::Html => self.render_html(),
            OutputFormat::Latex => bail!("LaTeX not supported yet"),
        }
    }

    fn render_html(self) -> Result<()> {
        let input = self.input.contents()?;
        let doc = Doc::new(&input)?;

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
        println!("{}", html);

        Ok(())
    }
}
