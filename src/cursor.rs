use std::collections::VecDeque;

use crate::{page::Page};

#[derive(Clone, Debug, Default)]
pub struct Cursor {
    pub position: (u16, u16),
}

impl Cursor {
    pub fn new() -> Self {
        Cursor { position: (0, 0) }
    }

    pub fn move_to(&mut self, x: u16, y: u16) {
        self.position = (x, y);
    }

    pub fn get_position(&self) -> &(u16, u16) {
        &self.position
    }

    #[allow(unused)]
    pub fn handle_interaction(&mut self, page: &mut Page) {
        todo!();
        let mut queue = VecDeque::new();  
        for element in page.body.iter_mut() {
            queue.push_back(element);
        }

        while let Some(node) = queue.pop_front() {
            if let Some(size) = node.get_size() {
                
                for neighbor in node.children.iter_mut() {
                    queue.push_back(neighbor);
                }
            }
        }
    }

    pub fn move_up(&mut self, steps: u16) {
        if self.position.1 < steps {
            self.position.1 = 0;
        } else {
            self.position.1 -= steps;
        }
    }
    pub fn move_down(&mut self, steps: u16) {
        if self.position.1 + steps >= crossterm::terminal::size().unwrap_or((0, 0)).1 {
            self.position.1 = crossterm::terminal::size().unwrap_or((0, 0)).1 - 1;
        } else {
            self.position.1 += steps;
        }
    }

    pub fn move_left(&mut self, steps: u16) {
        if self.position.0 < steps {
            self.position.0 = 0;
        } else {
            self.position.0 -= steps;
        }
    }
    pub fn move_right(&mut self, steps: u16) {
        if self.position.0 + steps >= crossterm::terminal::size().unwrap_or((0, 0)).0 {
            self.position.0 = crossterm::terminal::size().unwrap_or((0, 0)).0 - 1;
        } else {
            self.position.0 += steps;
        }
    }
}
