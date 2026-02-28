pub mod config;
pub mod int;
pub mod null;
pub mod text;
pub mod char;

pub use config::ConfigType;
pub use int::IntType;
pub use null::NullType;
pub use text::TextType;
pub use char::CharType;

use btmd_macro::ConfigLookup;
use enum_dispatch::enum_dispatch;
use serde_jsonc::Value;

#[enum_dispatch]
pub trait ValueType {
    fn parse(&self, value: &Value) -> ValueTypes;
}

#[enum_dispatch(ValueType)]
#[derive(ConfigLookup, Clone, Debug)]
pub enum ValueTypes {
    #[config_def("null")]
    Null(NullType),
    #[config_def("text")]
    Text(TextType),
    #[config_def("int")]
    Int(IntType),
    #[config_def("config")]
    Config(ConfigType),
    #[config_def("char")]
    Char(CharType),
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