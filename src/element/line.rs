use serde_jsonc::Value;

use crate::{
    content::{Content, Text},
    element::Element,
};

use std::{cell::RefCell, sync::LazyLock};

pub static LINE: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder, _, args: Vec<Value>, parent_size: &(u16, u16), _, _| {
            let char: String = args
                .get(0)
                .unwrap_or(&Value::String("─".to_string()))
                .as_str()
                .unwrap_or("─")
                .chars()
                .last()
                .unwrap_or('─')
                .to_string();

            if char == "\n" {
                return Content::new(
                    vec![Text::new_default("\n".to_string())],
                    false,
                    (parent_size.0, 1),
                    RefCell::new(holder.to_owned()),
                );
            }
            if char == " " {
                return Content::new(
                    vec![Text::new_default(String::new())],
                    false,
                    (parent_size.0, 1),
                    RefCell::new(holder.to_owned()),
                );
            }
            if char == "-" {
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
                vec![Text::new_default(char.repeat(parent_size.0 as usize))],
                true,
                (parent_size.0, 1),
                RefCell::new(holder.to_owned()),
            )
        },
        "line",
    )
});
