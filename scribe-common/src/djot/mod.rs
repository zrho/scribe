//! Utilities to process djot documents.

mod error;
mod frontmatter;
mod headings;
mod inkjet;
mod katex;

pub use error::ShowErrors;
pub use frontmatter::split_frontmatter;
pub use headings::DemoteHeadings;
pub use inkjet::{InkjetCode, InkjetCodeError};
pub use katex::{KatexMath, KatexMathError};
