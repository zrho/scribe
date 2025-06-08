use inkjet::{Highlighter, Language};
use jotdown::{Container, Event};
use thiserror::Error;
use tracing::trace;

/// Render code blocks to HTML using Inkjet.
///
/// Code blocks for languages that are not supported by Inkjet are left unmodified.
#[derive(Clone)]
pub struct InkjetCode<'a, I> {
    inner: I,
    highlighter: Highlighter,
    buffer: Vec<Event<'a>>,
}

impl<'a, I> InkjetCode<'a, I> {
    pub fn new(inner: I, highlighter: Highlighter) -> Self {
        Self {
            inner,
            highlighter,
            buffer: Vec::with_capacity(2),
        }
    }
}

impl<'a, I> Iterator for InkjetCode<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Result<Event<'a>, InkjetCodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.buffer.pop() {
            return Some(Ok(event));
        }

        let (language, attributes) = match self.inner.next()? {
            Event::Start(Container::CodeBlock { language }, attributes) => (language, attributes),
            event => return Some(Ok(event)),
        };

        trace!("code block with language `{}`", language);

        let mut code = String::new();

        loop {
            match self.inner.next()? {
                Event::End(_) => break,
                Event::Str(str) => code.push_str(&str),
                _ => return Some(Err(InkjetCodeError::Unexpected)),
            }
        }

        let Some(language) = Language::from_token(language) else {
            trace!("language `{}` not supported by Inkjet", language);

            self.buffer.extend([
                Event::End(Container::CodeBlock { language }),
                Event::Str(code.into()),
            ]);

            return Some(Ok(Event::Start(
                Container::CodeBlock { language },
                attributes,
            )));
        };

        let result = self
            .highlighter
            .highlight_to_string(language, &inkjet::formatter::Html, code);

        let result = match result {
            Ok(result) => result,
            Err(err) => return Some(Err(err.into())),
        };

        let result = format!("<pre><code>\n{}\n</code></pre>", result);

        self.buffer.extend([
            Event::End(Container::RawBlock { format: "html" }),
            Event::Str(result.into()),
        ]);

        Some(Ok(Event::Start(
            Container::RawBlock { format: "html" },
            attributes,
        )))
    }
}

/// Error produced by [`InkjetCode`].
#[derive(Debug, Error)]
pub enum InkjetCodeError {
    /// Error while rendering code block.
    #[error("failed to render code block with inkjet: {0}")]
    Inkjet(#[from] inkjet::InkjetError),
    /// Unexpected [`Event`] in code block.
    #[error("unexpected event in code block")]
    Unexpected,
}
