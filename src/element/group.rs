use serde_json::{Map, Value};
use std::cell::RefCell;
use std::sync::{Arc, LazyLock, RwLock};

use crate::content::ContentBuilder;
use crate::{content::Content, element::Element, page::Page, parse::parse_vec_to_vec};

pub static GROUP: LazyLock<Element> = LazyLock::new(|| {
    Element::new(
        |holder: &mut Element,
         page: &mut Page,
         args: Vec<Value>,
         parent_size: &(u16, u16),
         timer: &u32,
         pos: (u32, u32)| {
            let mut default_config: Map<String, Value> = Map::new();
            let config: Map<String, Value> = args
                .get(1)
                .unwrap_or(&Value::Object(Map::new()))
                .as_object()
                .unwrap_or(&default_config)
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();
            for (k, v) in config.iter() {
                default_config.insert(k.clone(), v.clone());
            }

            let width: i32 = parent_size.0 as i32;

            let mut border_builder: ContentBuilder = ContentBuilder::new();

            let mut i: u32 = 0;
            let mut lines: u16 = 0;

            // let body_raw: Vec<Value> = args
            //     .get(0)
            //     .unwrap_or(&Value::Array(vec![]))
            //     .as_array()
            //     .unwrap()
            //     .to_vec();

            // let body: Vec<Element> = parse_vec_to_vec(body_raw, &page.registry);

            let mut rendered_content: Vec<Content> = Vec::new();

            for element_rc in holder.children.iter() {
                let mut element = element_rc.write().unwrap();
                rendered_content.push(element.render(
                    page,
                    &(parent_size),
                    timer,
                    (
                        i as u32 % parent_size.0 as u32 + pos.0,
                        lines as u32 + pos.1,
                    ),
                ));
                for t in &rendered_content.last().unwrap().text {
                    let mut temp: String = String::new();
                    for char in t.text.chars() {
                        if char == '\n' {
                            if i % parent_size.0 as u32 != 0 {
                                temp.push_str(&*" ".repeat(
                                    (width as u32 - (i) % width as u32).try_into().unwrap(),
                                ));
                                i += width as u32 - (i - 1) % width as u32;
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    t.background_color,
                                );
                                temp = String::new();
                                lines += 1;
                            }
                        } else if i % parent_size.0 as u32 == 0 {
                            lines += 1;
                            i += 1;
                            temp.push(char);
                        } else if char == '\t' {
                            let spaces: usize = 4 - (i as usize - 1) % 4;
                            temp.push_str(&*" ".repeat(spaces));
                            i += spaces as u32;
                        } else {
                            temp.push(char);
                            i += 1;
                        }
                    }
                    border_builder.append_text(temp, t.foreground_color, t.background_color);
                }
            }

            if !(i % width as u32 == 0) {
                border_builder.append_text_default(
                    (&*" ".repeat((width as u32 - i % width as u32) as usize)).to_string(),
                );
            }

            border_builder.build(
                true,
                (parent_size.0, lines),
                RefCell::new(holder.to_owned()),
            )
        },
        vec![],
        |args: &Vec<Value>, page: &Page| -> Vec<Arc<RwLock<Element>>> {
            let res = parse_vec_to_vec(
                (*args
                    .get(0)
                    .unwrap_or(&Value::Array(vec![]))
                    .as_array()
                    .unwrap_or(&vec![]))
                .clone(),
                &page.registry,
            );
            res
        },
        "group",
        (0, 0),
    )
});
