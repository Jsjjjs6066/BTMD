use serde_jsonc::Value;

use crate::args::{ArgPreset, ArgType, ArgTypes};

#[derive(Clone, Default)]
pub struct ArgParser {
    preset: ArgPreset,
}

impl ArgParser {
    pub fn new(config_preset: ArgPreset) -> ArgParser {
        ArgParser {
            preset: config_preset,
        }
    }

    pub fn parse(&self, vec: Vec<Value>) -> Vec<ArgTypes> {
        let mut v = self.preset.vec.clone();
        for (val1, val2) in vec.iter().zip(v.iter_mut()) {
            *val2 = val2.parse(val1);
        }
        v
    }
}
