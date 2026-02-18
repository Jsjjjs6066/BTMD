use serde_jsonc::Value;
use std::{cell::RefCell, cmp::min};

use crate::{
    args::{self, ArgTypes, get_arg},
    args_parser,
    content::Content,
    element::Element,
};
use btmd_macro::unwrap_arg;

use std::sync::LazyLock;

pub static PARA: LazyLock<Element> = LazyLock::new(|| {
    Element::new_default(
        |holder, _, args: Vec<Value>, parent_size: &(u16, u16), _, _| {
            let arg_parser = args_parser!(get_arg("text"), get_arg("config"));
            let args_parsed = arg_parser.parse(args);
            let text: args::TextType = unwrap_arg!(args_parsed.first().unwrap(), Text);
            let _config: args::ConfigType = unwrap_arg!(args_parsed.last().unwrap(), Config);
            Content::new(
                vec![text.0.clone()],
                false,
                (
                    min(
                        (text.0.text.chars().count() as u16).saturating_sub_signed(1),
                        parent_size.0,
                    ),
                    text.0.text.lines().count() as u16,
                ),
                RefCell::new(holder.to_owned()),
            )
        },
        "para",
    )
});
