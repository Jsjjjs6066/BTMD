use crate::content::Content;

#[derive(Clone)]
pub struct ProcessedContent {
    pub content: Content,
    pub position: (u16, u16),
}

impl ProcessedContent {
    pub fn new(content: Content, position: (u16, u16)) -> Self {
        Self { content, position }
    }
}
