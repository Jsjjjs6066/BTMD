use crate::{content::Content, element::Element};

pub struct ProcessedContent {
    pub content: Content,
    pub position: (u16, u16),
    pub element: Element,
}

impl ProcessedContent {
    pub fn new(content: Content, position: (u16, u16), element: Element) -> Self {
        Self {
            content,
            position,
            element,
        }
    }
}