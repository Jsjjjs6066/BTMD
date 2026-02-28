use serde_jsonc::Value;

use crate::{args::ArgPreset, values::{ValueType, ValueTypes}};

#[derive(Clone, Default)]
pub struct ArgParser {
    pub(crate) preset: ArgPreset,
}

impl ArgParser {
    pub fn new(config_preset: ArgPreset) -> ArgParser {
        ArgParser {
            preset: config_preset,
        }
    }

    pub fn parse(&self, vec: Vec<Value>) -> Vec<ValueTypes> {
        let mut v = self.preset.vec.clone();
        let mut vec_iter = vec.iter()
            .map(Some)
            .chain(std::iter::repeat_with(|| None));
        for (val1, val2) in v.iter_mut().zip(vec_iter.by_ref()) {
            if let None = val2 {
                *val1 = val1.parse(&Value::Null);
                continue;
            }
            let val1u = val1.parse(val2.unwrap());
            *val1 = val1u;
        }
        v
    }
}
