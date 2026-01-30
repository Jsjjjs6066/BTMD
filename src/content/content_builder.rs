use crossterm::style::Color;

use crate::content::{Content, Text, processed_content::ProcessedContent};

pub struct ContentBuilder {
    pub content: Vec<Text>,
    pub children: Vec<ProcessedContent>,
}

impl ContentBuilder {
    pub fn new() -> Self {
        ContentBuilder {
            content: Vec::new(),
            children: Vec::new(),
        }
    }

    pub fn append_text(&mut self, text: String, foreground_color: Color, background_color: Color) {
        self.content
            .push(Text::new(text, foreground_color, background_color))
    }
    pub fn append_text_default(&mut self, text: String) {
        self.content.push(Text::new_default(text))
    }

    pub fn add_child(mut self, child: ProcessedContent) -> Self {
        self.children.push(child);
        self
    }
    pub fn add_children(mut self, children: Vec<ProcessedContent>) -> Self {
        for child in children {
            self.children.push(child.clone());
        }
        self
    }

    pub fn build(self, rerender_needed: bool, size: (u16, u16)) -> Content {
        Content {
            text: self.content,
            rerender_needed: rerender_needed,
            size,
            children: self.children,
            current_text_index: 0,
            current_char_index: 0,
            position: None,
        }
    }
}
