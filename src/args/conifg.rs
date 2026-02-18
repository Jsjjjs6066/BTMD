use std::collections::HashMap;

use serde_jsonc::Value;

use crate::{
    args::{ArgType, ArgTypes},
    config::{ConfigPreset, ValueTypes, config_parser::ConfigParser},
};

#[derive(Clone, Default)]
pub struct ConfigType(ConfigPreset, pub HashMap<String, ValueTypes>);

impl ArgType for ConfigType {
    fn parse(&self, value: &Value) -> ArgTypes {
        ArgTypes::Config(Self(
            self.0.clone(),
            match value {
                Value::Object(o) => {
                    let config_parser = ConfigParser::new(self.0.to_owned());
                    config_parser.parse(o.clone())
                }
                _ => Default::default(),
            },
        ))
    }
}
