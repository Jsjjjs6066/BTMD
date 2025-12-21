use crate::{content::Content, element::Element};

#[derive(Clone)]
pub struct ProcessedContent<'a> {
    pub content: Content<'a>,
    pub position: (u16, u16),
    pub element: &'a Element<'a>,
}

impl<'a> ProcessedContent<'a> {
    pub fn new(content: Content<'a>, position: (u16, u16), element: &'a Element) -> Self {
        Self {
            content,
            position,
            element,
        }
    }
}