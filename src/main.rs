use anyhow::Result;
use clap::Parser;
use clap_stdin::FileOrStdin;
use cli::{Cli, Command};
use doc::Doc;
use inkjet::Highlighter;
use jotdown::html::render_to_string;
use passes::{error::ShowError, inkjet::InkjetCode, katex::KatexMath};

pub mod cli;
pub mod doc;
pub mod passes;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Render(render) => {
            println!("{:?}", render.output_format);
            println!("{}", render.input.contents()?);
        }
    }

    Ok(())
}

// fn main() -> Result<()> {
//     let file = include_str!("../test.dj");
//     let doc = Doc::new(file)?;

//     let katex_opts = katex::Opts::builder()
//         .macros(doc.header.math.macros.clone())
//         .build()
//         .unwrap();

//     let parser = Parser::new(doc.body);

//     let parser = KatexMath::new(parser, katex_opts);
//     let parser = ShowError::new(parser);

//     let parser = InkjetCode::new(parser, Highlighter::new());
//     let parser = ShowError::new(parser);

//     println!("{:?}", doc.header);
//     println!("{}", render_to_string(parser));
//     Ok(())
// }
