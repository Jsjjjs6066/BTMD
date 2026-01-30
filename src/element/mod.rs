use crate::{content::Content, page::Page};

pub mod registry;
use serde_json::Value;
use std::fmt::Debug;

pub mod border;
pub mod group;
pub mod heading;
pub mod label;
pub mod line;
pub mod new_line;
pub mod none;
pub mod para;

pub use border::BORDER;
pub use group::GROUP;
pub use heading::HEADING;
pub use label::LABEL;
pub use line::LINE;
pub use new_line::NEW_LINE;
pub use none::NONE;
pub use para::PARA;

#[derive(Clone)]
pub struct Element {
    render_func: fn(
        holder: &mut Element,
        page: &mut Page,
        args: Vec<Value>,
        parent_size: &(u16, u16),
        timer: &u32,
    ) -> Content,
    pub args: Vec<Value>,
    pub children: Vec<Element>,
    prepare_children_func: fn(&Vec<Value>, &Page) -> Vec<Element>,
    element_tag: &'static str,
    on_hover_func: fn(
        holder: &mut Element,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
    ),
    size: Option<(u16, u16)>,
    position: Option<(u16, u16)>,
}

impl Element {
    pub fn new(
        render_func: fn(
            holder: &mut Element,
            page: &mut Page,
            args: Vec<Value>,
            parent_size: &(u16, u16),
            timer: &u32,
        ) -> Content,
        args: Vec<Value>,
        prepare_children_function: fn(&Vec<Value>, &Page) -> Vec<Element>,
        element_tag: &'static str,
    ) -> Self {
        Element {
            render_func,
            args,
            children: Vec::new(),
            prepare_children_func: prepare_children_function,
            element_tag,
            on_hover_func: |_, _, _, _| {},
            size: None,
            position: None,
        }
    }
    pub fn new_default(
        render_func: fn(
            holder: &mut Element,
            page: &mut Page,
            args: Vec<Value>,
            parent_size: &(u16, u16),
            timer: &u32,
        ) -> Content,
        element_tag: &'static str,
    ) -> Self {
        Element {
            render_func,
            args: Vec::new(),
            children: Vec::new(),
            prepare_children_func: |_, _| -> Vec<Element> { return Vec::new() },
            element_tag,
            on_hover_func: |_, _, _, _| {},
            size: None,
            position: None,
        }
    }
    pub fn new_from(&self, args: Vec<Value>) -> Self {
        let mut new_element = self.clone();
        new_element.args = args;
        new_element.children = Vec::new();
        new_element
    }

    fn prepare_children(&mut self, page: &Page) {
        if self.children.is_empty() {
            self.children = (self.prepare_children_func)(&self.args, page);
        }
    }

    pub fn render(&mut self, page: &mut Page, parent_size: &(u16, u16), timer: &u32) -> Content {
        self.prepare_children(page);
        let c: Content = (self.render_func)(self, page, self.args.clone(), parent_size, timer);
        self.size = Some(c.size);
        c
    }
    pub fn rerender(&mut self, page: &mut Page, parent_size: &(u16, u16), timer: &u32) -> Content {
        let c: Content = (self.render_func)(self, page, self.args.clone(), parent_size, timer);
        self.size = Some(c.size);
        c
    }

    pub fn on_hover(&mut self, page: &mut Page, parent_size: &(u16, u16), timer: &u32) {
        (self.on_hover_func)(self, page, parent_size, timer)
    }

    pub fn set_on_hover_func(&mut self, on_hover_func: fn(
        holder: &mut Element,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
    )) {
        self.on_hover_func = on_hover_func;
    }

    pub fn get_size(&self) -> Option<(u16, u16)> {
        self.size
    }

    pub fn get_position(&self) -> Option<(u16, u16)> {
        self.position
    }
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("args", &self.args)
            .field("children", &self.children)
            .field("element_tag", &self.element_tag)
            .field("size", &self.size)
            .finish()
    }
}
