pub mod arg_parser;
pub mod conifg;
pub mod null;
pub mod text;

pub use crate::args::conifg::ConfigType;
pub use arg_parser::ArgParser;
pub use null::NullType;
pub use text::TextType;

use enum_dispatch::enum_dispatch;
use serde_jsonc::Value;
use btmd_macro::ArgLookup;

#[enum_dispatch]
pub trait ArgType {
    fn parse(&self, value: &Value) -> ArgTypes;
}

#[enum_dispatch(ArgType)]
#[derive(ArgLookup, Clone)]
pub enum ArgTypes {
    #[arg_def("null")]
    Null(NullType),
    #[arg_def("text")]
    Text(TextType),
    #[arg_def("config")]
    Config(ConfigType),
}

impl Default for ArgTypes {
    fn default() -> Self {
        ArgTypes::Null(NullType)
    }
}
impl Default for &ArgTypes {
    fn default() -> Self {
        &ArgTypes::Null(NullType)
    }
}

pub fn get_arg(name: &str) -> ArgTypes {
    ArgTypes::get_arg(name)
}

#[derive(Clone, Default)]
pub struct ArgPreset {
    vec: Vec<ArgTypes>,
}

impl ArgPreset {
    pub fn new(vec: Vec<ArgTypes>) -> Self {
        Self { vec }
    }
}

#[macro_export]
macro_rules! args_parser {
    ($($name:expr),+) => {
        crate::args::ArgParser::new(crate::args::ArgPreset::new(vec![$($name),+]))
    };
}