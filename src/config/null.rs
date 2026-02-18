use serde_jsonc::Value;

use crate::config::{ValueType, ValueTypes};

#[derive(Clone, Default)]
pub struct NullType;

impl ValueType for NullType {
    fn parse(&self, _: &Value) -> ValueTypes {
        ValueTypes::Null(Self)
    }
}
