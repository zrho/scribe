use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Doc<'a> {
    pub header: Header,
    pub body: &'a str,
}

impl<'a> Doc<'a> {
    pub fn new(source: &'a str) -> Result<Self, DocError> {
        match split_source(source) {
            Some((header, body)) => {
                let header = serde_yaml_ng::from_str(header)?;
                Ok(Self { header, body })
            }
            None => Ok(Self {
                header: Header::default(),
                body: source,
            }),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Header {
    #[serde(default, rename = "type")]
    pub content_type: Option<String>,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub date: Option<String>,
    #[serde(default)]
    pub math: MathHeader,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct MathHeader {
    #[serde(default)]
    pub macros: HashMap<String, String>,
}

fn split_source<'a>(source: &'a str) -> Option<(&'a str, &'a str)> {
    let rest = source.strip_prefix("---\n")?;
    rest.split_once("\n---\n")
}

#[derive(Debug, Error)]
pub enum DocError {
    #[error("could not deserialize frontmatter")]
    DeserializeFrontmatter(#[from] serde_yaml_ng::Error),
}
