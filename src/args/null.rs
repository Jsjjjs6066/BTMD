use serde_jsonc::Value;

use crate::args::{ArgType, ArgTypes};

#[derive(Clone, Default)]
pub struct NullType;

impl ArgType for NullType {
    fn parse(&self, _: &Value) -> ArgTypes {
        ArgTypes::Null(Self)
    }
}
