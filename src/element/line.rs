use btmd_macro::unwrap_val;
use serde_jsonc::Value;

use crate::{
    args_parser, config_preset, content::{Content, Text}, element::Element, logger, values::{CharType, ValueTypes}
};

use std::{cell::RefCell, sync::LazyLock};

pub static LINE: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder, _, args: Vec<Value>, parent_size: &(u16, u16), _, _| {
            #[allow(unused)]
            let config_parser = config_preset!();
            let arg_parser = args_parser!(ValueTypes::Char(CharType('─')));
            let args_parsed = arg_parser.parse(args);
            let char: char = unwrap_val!(args_parsed.first().unwrap(), Char).0;
            logger::write_log(format!("{:#?}", args_parsed).as_bytes()).unwrap();
            if char == '\n' {
                return Content::new(
                    vec![Text::new_default("\n".to_string())],
                    false,
                    (parent_size.0, 1),
                    RefCell::new(holder.to_owned()),
                );
            }
            if char == ' ' {
                return Content::new(
                    vec![Text::new_default(String::new())],
                    false,
                    (parent_size.0, 1),
                    RefCell::new(holder.to_owned()),
                );
            }
            if char == '-' {
                return Content::new(
                    vec![Text::new_default(
                        "─".to_string().repeat(parent_size.0 as usize),
                    )],
                    true,
                    (parent_size.0, 1),
                    RefCell::new(holder.to_owned()),
                );
            }
            Content::new(
                vec![Text::new_default(char.to_string().repeat(parent_size.0 as usize))],
                true,
                (parent_size.0, 1),
                RefCell::new(holder.to_owned()),
            )
        },
        "line",
    )
});
