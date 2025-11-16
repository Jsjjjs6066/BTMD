use serde_json::Value;

use crate::{cursor::Cursor, element::{Element, registry::{ElementRegistry}}};

#[derive(Clone)]
pub struct Page {
	pub title: String,
	pub body: Vec<Element>,
	pub body_raw: Value,
	pub cursor: Cursor,
	pub registry: ElementRegistry,
}

impl Page {
	pub fn new(title: String, body: Vec<Element>, body_raw: Value, registry: ElementRegistry) -> Self {
		Page {title, body, body_raw, cursor: Cursor::new(), registry}
	}
}
