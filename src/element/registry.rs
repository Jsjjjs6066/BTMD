use std::collections::HashMap;

use serde_json::de;

use crate::element::Element;

#[derive(Clone)]
pub struct ElementRegistry {
	registry: HashMap<String, Element>,
}

impl ElementRegistry {
	pub fn new() -> Self {
		Self {
			registry: HashMap::new(),
		}
	}

	pub fn register_element(&mut self, name: String, element: &Element) {
		self.registry.insert(name, element.to_owned().clone());
	}

	pub fn get_element(&self, name: &str) -> Element {
		self.registry.get(name)
			.or_else(|| self.registry.get("none"))
			.cloned().unwrap()
	}

	pub fn add_alias(&mut self, alias: String, target: &str) {
		let element = self.get_element(target);
		self.register_element(alias, &element);
	}
}

// use std::{collections::HashMap, sync::{LazyLock, Mutex}};

// use crate::element::Element;

// static ELEMENT_REGISTRY: LazyLock<Mutex<HashMap<String, Element>>> = LazyLock::new(|| Mutex::new(HashMap::new()));
	

// pub fn register_element(name: String, element: &Element) {
// 	let mut registry = ELEMENT_REGISTRY.lock().unwrap();
// 	registry.insert(name, element.to_owned().clone());
// }

// pub fn get_element(name: &str) -> Element {
// 	let registry = ELEMENT_REGISTRY.lock().unwrap();
// 	registry.get(name)
// 		.or_else(|| registry.get("none"))
// 		.cloned().unwrap()
// }

// pub fn add_alias(alias: String, target: &str) {
// 	register_element(alias, &get_element(target));
// }