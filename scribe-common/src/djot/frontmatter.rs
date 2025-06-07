pub fn split_frontmatter<'a>(source: &'a str) -> (Option<&'a str>, &'a str) {
    let split = source
        .strip_prefix("---\n")
        .and_then(|rest| rest.split_once("\n---\n"));

    match split {
        Some((frontmatter, body)) => (Some(frontmatter), body),
        None => (None, source),
    }
}
