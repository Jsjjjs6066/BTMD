use serde_json::Value;

use crate::{cursor::Cursor, element::{Element, registry::{ElementRegistry}}};

#[derive(Clone)]
pub struct Page<'a> {
	pub title: String,
	pub body: Vec<Element<'a>>,
	pub body_raw: Value,
	pub cursor: Cursor,
	pub registry: ElementRegistry<'a>,
}

impl<'a> Page<'a> {
	pub fn new(title: String, body: Vec<Element<'a>>, body_raw: Value, registry: ElementRegistry<'a>) -> Self {
		Page {title, body, body_raw, cursor: Cursor::new(), registry}
	}
}
