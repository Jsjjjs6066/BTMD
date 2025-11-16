pub mod text;
pub mod content_builder;
pub mod processed_content;

use std::ops::Deref;

pub use text::Text;
pub use content_builder::ContentBuilder;

use crate::content::processed_content::ProcessedContent;

#[derive(Clone)]
pub struct Content {
    pub text: Vec<Text>,
    pub rerender_needed: bool,
    pub size: (u16, u16),
    pub children: Vec<ProcessedContent>,
}

impl Content {
    pub fn new(text: Vec<Text>, rerender_needed: bool, 
            size: (u16, u16)) -> Content {
        Content {text, rerender_needed, size, children: Vec::new()}
    }

    pub fn add_child(mut self, child: ProcessedContent) -> Self {
        self.children.push(child);
        self
    }
    pub fn add_children(mut self, children: Vec<ProcessedContent>) -> Self {
        for child in children.deref() {
            self.children.push(child.clone());
        }
        self
    }

    pub fn render(&self, parent_size: &(u16, u16)) -> String {
        self.text.iter().map(|content_type: &Text| content_type.render()).collect::<Vec<String>>().join("")
    }
}