use jotdown::{Container, Event};

/// Demote the headings in the document by a fixed offset.
pub struct DemoteHeadings<I> {
    inner: I,
    offset: u16,
}

impl<I> DemoteHeadings<I> {
    pub fn new(inner: I, offset: u16) -> Self {
        Self { inner, offset }
    }

    fn map_container<'a>(&self, container: Container<'a>) -> Container<'a> {
        match container {
            Container::Heading {
                level,
                has_section,
                id,
            } => {
                let level = level.saturating_add(self.offset);
                Container::Heading {
                    level,
                    has_section,
                    id,
                }
            }
            container => container,
        }
    }
}

impl<'a, I> Iterator for DemoteHeadings<I>
where
    I: Iterator<Item = Event<'a>>,
{
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(match self.inner.next()? {
            Event::Start(container, attributes) => {
                let container = self.map_container(container);
                Event::Start(container, attributes)
            }
            Event::End(container) => {
                let container = self.map_container(container);
                Event::End(container)
            }
            event => event,
        })
    }
}
