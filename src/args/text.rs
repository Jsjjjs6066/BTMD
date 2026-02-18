use serde_jsonc::{Value, to_string};

use crate::{
    args::{ArgType, ArgTypes},
    content::Text,
};

#[derive(Default, Clone)]
pub struct TextType(pub Text);

impl ArgType for TextType {
    fn parse(&self, value: &Value) -> ArgTypes {
        ArgTypes::Text(match value {
            Value::Null | Value::Array(_) | Value::Object(_) => TextType(Default::default()),
            Value::String(s) => TextType(Text::new_default(s.clone())),
            Value::Number(n) => TextType(Text::new_default(to_string(n).unwrap_or_default())),
            Value::Bool(b) => TextType(Text::new_default(to_string(b).unwrap_or_default())),
        })
    }
}
