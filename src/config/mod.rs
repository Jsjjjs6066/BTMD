pub mod config_parser;
pub mod null;
pub mod text;
pub use null::NullType;
pub use text::TextType;

use enum_dispatch::enum_dispatch;
use serde_jsonc::Value;
use std::collections::HashMap;

#[enum_dispatch]
pub trait ValueType {
    fn parse(&self, value: &Value) -> ValueTypes;
}

#[enum_dispatch(ValueType)]
#[derive(Clone)]
pub enum ValueTypes {
    Null(NullType),
    Text(TextType),
}

impl Default for ValueTypes {
    fn default() -> Self {
        ValueTypes::Null(NullType)
    }
}
impl Default for &ValueTypes {
    fn default() -> Self {
        &ValueTypes::Null(NullType)
    }
}

#[derive(Clone, Default)]
pub struct ConfigPreset {
    map: HashMap<String, ValueTypes>,
}

impl ConfigPreset {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add_value(&mut self, key: String, value_type: ValueTypes) {
        self.map.insert(key, value_type);
    }

    pub fn get_type(&self, key: &str) -> &ValueTypes {
        self.map.get(key).unwrap_or(Default::default())
    }
}
