use serde::Deserialize;
use thiserror::Error;

/// Split off the frontmatter string, if any.
pub fn split_frontmatter<'a>(source: &'a str) -> (Option<&'a str>, &'a str) {
    let split = source
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"));

    match split {
        Some((frontmatter, body)) => (Some(frontmatter), body),
        None => (None, source),
    }
}

/// Parse the YAML frontmatter string, if any.
pub fn parse_frontmatter<'a, T>(source: &'a str) -> Result<(T, &'a str), FrontmatterError>
where
    T: Default + Deserialize<'a>,
{
    let (frontmatter, body) = split_frontmatter(source);

    let frontmatter = match frontmatter {
        Some(frontmatter) => serde_yaml_ng::from_str(frontmatter).map_err(FrontmatterError)?,
        None => T::default(),
    };

    Ok((frontmatter, body))
}

#[derive(Debug, Error)]
#[error("Error while parsing frontmatter.")]
pub struct FrontmatterError(#[source] serde_yaml_ng::Error);
