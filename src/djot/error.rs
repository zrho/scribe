use std::fmt::Display;

use jotdown::{Attributes, Container, Event};
use tracing::warn;

/// Display errors in the document.
///
/// Errors are translated into a div with the `error` class. The error's
/// [`Display`] implementation is used to generate the error message.
#[derive(Debug, Clone)]
pub struct ShowErrors<'a, I> {
    inner: I,
    buffer: Vec<Event<'a>>,
}

impl<'a, I> ShowErrors<'a, I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            buffer: Vec::with_capacity(2),
        }
    }
}

impl<'a, I, E> Iterator for ShowErrors<'a, I>
where
    I: Iterator<Item = Result<Event<'a>, E>>,
    E: Display,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(event) = self.buffer.pop() {
            return Some(event);
        }

        let error = match self.inner.next()? {
            Ok(event) => return Some(event),
            Err(error) => error,
        };

        warn!("{}", error);

        self.buffer.extend([
            Event::End(Container::Div { class: "error" }),
            Event::Str(error.to_string().into()),
        ]);

        Some(Event::Start(
            Container::Div { class: "error" },
            Attributes::new(),
        ))
    }
}
