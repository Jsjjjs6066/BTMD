use std::collections::HashMap;

use serde_jsonc::{Map, Value};

use crate::config::ConfigPreset;
use crate::values::{ValueType, ValueTypes};

#[derive(Clone, Default)]
pub struct ConfigParser {
    preset: ConfigPreset,
}

impl ConfigParser {
    pub fn new(config_preset: ConfigPreset) -> ConfigParser {
        ConfigParser {
            preset: config_preset,
        }
    }

    pub fn parse(&self, map: Map<String, Value>) -> HashMap<String, ValueTypes> {
        let mut hm = self.preset.map.clone();
        for (k, v) in map {
            hm.insert(k.clone(), hm.get(&k).unwrap_or_default().parse(&v));
        }
        hm
    }
}
