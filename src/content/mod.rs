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
    current_text_index: usize,
    current_char_index: usize,
}

impl Content {
    pub fn new(text: Vec<Text>, rerender_needed: bool, 
            size: (u16, u16)) -> Content {
        Content {text, rerender_needed, size, children: Vec::new(), current_text_index: 0, current_char_index: 0}
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

impl Iterator for Content {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // Check if current_text_index is out of bounds
            if self.current_text_index >= self.text.len() {
                return None;
            }

            // Get the current text element
            let current_text = &self.text[self.current_text_index];
            let chars: Vec<char> = current_text.text.chars().collect();

            // Check if current_char_index is within the current text's characters
            if self.current_char_index < chars.len() {
                let ch = chars[self.current_char_index];
                self.current_char_index += 1;
                return Some(ch);
            } else {
                // Move to the next text element
                self.current_text_index += 1;
                self.current_char_index = 0;
            }
        }
    }
}