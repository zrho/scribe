use std::fmt::Display;

use jotdown::{Attributes, Container, Event};

#[derive(Debug, Clone)]
pub struct ShowError<'a, I> {
    inner: I,
    buffer: Vec<Event<'a>>,
}

impl<'a, I> ShowError<'a, I> {
    pub fn new(inner: I) -> Self {
        Self {
            inner,
            buffer: Vec::with_capacity(2),
        }
    }
}

impl<'a, I, E> Iterator for ShowError<'a, I>
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
