pub mod arg_parser;

use crate::values::ValueTypes;
pub use arg_parser::ArgParser;

#[derive(Clone, Default)]
pub struct ArgPreset {
    pub(crate) vec: Vec<ValueTypes>,
}

impl ArgPreset {
    pub fn new(vec: Vec<ValueTypes>) -> Self {
        Self { vec }
    }
}

#[macro_export]
macro_rules! args_parser {
    ($($name:expr),+) => {
        $crate::args::ArgParser::new($crate::args::ArgPreset::new(vec![$($name),+]))
    };
}
