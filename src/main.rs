use anyhow::Result;
use clap::Parser as _;
use cli::Cli;

pub mod cli;
pub mod djot;
pub mod doc;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    Cli::parse().run()
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
