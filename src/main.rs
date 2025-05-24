use anyhow::Result;
use doc::Doc;
use inkjet::Highlighter;
use jotdown::{Parser, html::render_to_string};
use passes::{error::ShowError, inkjet::InkjetCode, katex::KatexMath};

pub mod doc;
pub mod passes;

fn main() -> Result<()> {
    let file = include_str!("../test.dj");
    let doc = Doc::new(file)?;

    let katex_opts = katex::Opts::builder()
        .macros(doc.header.math.macros.clone())
        .build()
        .unwrap();

    let parser = Parser::new(doc.body);

    let parser = KatexMath::new(parser, katex_opts);
    let parser = ShowError::new(parser);

    let parser = InkjetCode::new(parser, Highlighter::new());
    let parser = ShowError::new(parser);

    println!("{:?}", doc.header);
    println!("{}", render_to_string(parser));
    Ok(())
}
