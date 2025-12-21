use crate::{
    element::{
        Element,
        registry::{self, ElementRegistry},
    },
    import_default_elements,
    page::Page,
};
use serde_json::Value;

pub fn parse_json_to_page<'a>(json_page: Value) -> Page<'a> {
    let title: String = json_page["title"].as_str().unwrap_or("Page").to_string();
    let body_unparsed: Vec<Value> = json_page["body"].as_array().unwrap_or(&Vec::new()).to_vec();

    let mut body: Vec<Element<'a>> = Vec::with_capacity(body_unparsed.len());

    let mut registry: ElementRegistry<'a> = ElementRegistry::new();
    import_default_elements(&mut registry);

    for element in body_unparsed {
        if let Some(arr) = element.as_array() {
            if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
                let args: Vec<Value> = arr[1..].to_vec();
                let element_instance: Element = registry.get_element(element_type).new_from(args);
                body.push(element_instance);
            }
        }
    }

    Page::new(title, body, json_page["body"].clone(), registry)
}
pub fn parse_str_to_page(input: &str) -> Page {
    let json_page: Value = serde_json::from_str(input).unwrap();
    parse_json_to_page(json_page)
}

pub fn parse_vec_to_vec<'a>(input: Vec<Value>, registry: &ElementRegistry<'a>) -> Vec<Element<'a>> {
    let mut body: Vec<Element<'a>> = Vec::with_capacity(input.len());

    for element in input {
        if let Some(arr) = element.as_array() {
            if let Some(element_type) = arr.get(0).and_then(|v: &Value| v.as_str()) {
                let args: Vec<Value> = arr[1..].to_vec();
                let element_instance: Element = registry.get_element(element_type).new_from(args);
                body.push(element_instance);
            }
        }
    }

    body
}
pub fn parse_str_to_vec<'a>(input: &str, registry: &ElementRegistry<'a>) -> Vec<Element<'a>> {
    let elements: Vec<Value> = serde_json::from_str(input)
        .unwrap_or(Value::Array(vec![]))
        .as_array()
        .unwrap_or(&Vec::new())
        .to_vec();
    parse_vec_to_vec(elements, registry)
}
