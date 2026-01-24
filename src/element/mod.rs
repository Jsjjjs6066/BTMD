use crate::{content::Content, page::Page};
use std::marker::PhantomData;
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
pub struct Element<'a> {
    render_func: for<'b> fn(
        holder: &'b mut Element,
        page: &mut Page,
        args: Vec<Value>,
        parent_size: &(u16, u16),
        timer: &u32,
    ) -> Content<'b>,
    pub args: Vec<Value>,
    pub children: Vec<Element<'a>>,
    prepare_children_func: fn(&Vec<Value>, &Page) -> Vec<Element<'a>>,
    element_tag: &'static str,
    _marker: PhantomData<&'a ()>,
}

impl<'a> Element<'a> {
    pub fn new(
        render_func: for<'b> fn(
            holder: &'b mut Element,
            page: &mut Page,
            args: Vec<Value>,
            parent_size: &(u16, u16),
            timer: &u32,
        ) -> Content<'b>,
        args: Vec<Value>,
        prepare_children_function: fn(&Vec<Value>, &Page) -> Vec<Element<'a>>,
        element_tag: &'static str,
    ) -> Self {
        Element {
            render_func,
            args,
            children: Vec::new(),
            prepare_children_func: prepare_children_function,
            element_tag,
            _marker: PhantomData,
        }
    }
    pub fn new_default(
        render_func: for<'b> fn(
            holder: &'b mut Element,
            page: &mut Page,
            args: Vec<Value>,
            parent_size: &(u16, u16),
            timer: &u32,
        ) -> Content<'b>,
        element_tag: &'static str,
    ) -> Self {
        Element {
            render_func,
            args: Vec::new(),
            children: Vec::new(),
            prepare_children_func: |args: &Vec<Value>, _| -> Vec<Element<'a>> { return Vec::new() },
            element_tag,
            _marker: PhantomData,
        }
    }
    pub fn new_from(&self, args: Vec<Value>) -> Self {
        let mut new_element = self.clone();
        new_element.args = args;
        new_element.children = Vec::new();
        new_element
    }

    fn prepare_children(&mut self, page: &Page) {
        self.children = (self.prepare_children_func)(&self.args, page);
    }

    pub fn render<'b>(
        &'b mut self,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
    ) -> Content<'b> {
        self.prepare_children(page);
        (self.render_func)(self, page, self.args.clone(), parent_size, timer)
    }
    pub fn rerender<'b>(
        &'b mut self,
        page: &mut Page,
        parent_size: &(u16, u16),
        timer: &u32,
    ) -> Content<'b> {
        (self.render_func)(self, page, self.args.clone(), parent_size, timer)
    }
}

impl Debug for Element<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element")
            .field("args", &self.args)
            .field("children", &self.children)
            .field("element_tag", &self.element_tag)
            .finish()
    }
}