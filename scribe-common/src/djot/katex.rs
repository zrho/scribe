use jotdown::{Container, Event};
use katex::Opts;
use thiserror::Error;
use tracing::trace;

/// Render math to HTML using KaTeX.
#[derive(Debug, Clone)]
pub struct KatexMath<'a, I> {
    inner: I,
    opts: Opts,
    buffer: Vec<Event<'a>>,
}

impl<'a, I> KatexMath<'a, I> {
    pub fn new(inner: I, opts: Opts) -> Self {
        Self {
            inner,
            opts,
            buffer: Vec::with_capacity(2),
        }
    }
}

impl<'a, I> Iterator for KatexMath<'a, I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Result<Event<'a>, KatexMathError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.buffer.pop() {
            return Some(Ok(event));
        };

        let (display, attributes) = match self.inner.next()? {
            Event::Start(Container::Math { display }, attributes) => (display, attributes),
            event => return Some(Ok(event)),
        };

        if display {
            trace!("found display math");
        } else {
            trace!("found inline math");
        }

        let mut math = String::new();

        loop {
            match self.inner.next()? {
                Event::End(_) => break,
                Event::Str(str) => math.push_str(&str),
                _ => return Some(Err(KatexMathError::Unexpected)),
            }
        }

        self.opts.set_display_mode(display);
        let result = katex::render_with_opts(&math, &self.opts);

        let rendered = match result {
            Ok(rendered) => rendered,
            Err(err) => return Some(Err(err.into())),
        };

        self.buffer.extend([
            Event::End(Container::RawBlock { format: "html" }),
            Event::Str(rendered.into()),
        ]);

        Some(Ok(Event::Start(
            Container::RawBlock { format: "html" },
            attributes,
        )))
    }
}

/// Error produced by [`KatexMath`].
#[derive(Debug, Clone, Error)]
pub enum KatexMathError {
    /// Error while rendering KaTeX math.
    #[error("failed to render katex math: {0}")]
    Katex(#[from] katex::Error),
    /// Unexpected [`Event`] in math block.
    #[error("unexpected event in math block")]
    Unexpected,
}
