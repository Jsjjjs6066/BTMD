use std::cmp::{max, min};

use serde_jsonc::{Value};

use crate::{values::{ValueType, ValueTypes}, int::Int};

#[derive(Clone, Default, Debug)]
pub struct IntType {
    pub int: Int,
    pub min: Int,
    pub max: Int,
}

impl ValueType for IntType {
    fn parse(&self, value: &Value) -> ValueTypes {
        ValueTypes::Int(IntType {
            int: max(min(self.max.to_owned(), Int::from_int(value.to_owned(), self.int.to_owned())), self.min.to_owned()),
            min: self.min.to_owned(),
            max: self.max.to_owned(),
        })
    }
}
