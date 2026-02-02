use serde_json::{Map, Value, json};

use crate::{
    content::{Content, ContentBuilder},
    element::Element,
    logger,
    page::Page,
    parse::parse_vec_to_vec,
};

use crossterm::style::Color;
use std::{cell::RefCell, sync::LazyLock};

pub static BORDER: LazyLock<Element> = LazyLock::new(|| {
    let mut e = Element::new(
        |holder: &mut Element,
         page: &mut Page,
         args: Vec<Value>,
         parent_size: &(u16, u16),
         timer: &u32| {
            let mut default_config: Map<String, Value> = Map::new();
            default_config.insert("min-height".to_string(), Value::Number(0.into()));
            default_config.insert("connect-to-horizontal-chars".to_string(), Value::Bool(true));
            default_config.insert("color".to_string(), Value::String("default".to_string()));
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

            if default_config.get("min-height").is_none() {
                default_config.insert("min-height".to_string(), Value::Number(0.into()));
            } else if default_config.get("min-height").unwrap().is_string() {
                if default_config.get("min-height").unwrap().as_str().unwrap() == "auto" {
                    default_config.insert("min-height".to_string(), Value::Number(0.into()));
                } else if default_config.get("min-height").unwrap().as_str().unwrap() == "max" {
                    default_config.insert(
                        "min-height".to_string(),
                        Value::Number((parent_size.1 - 2).into()),
                    );
                }
            }

            let should_connect_to_horizontal_chars: bool = default_config
                .get("connect-to-horizontal-chars")
                .unwrap_or(&Value::Bool(true))
                .as_bool()
                .unwrap_or(true);

            let width: usize = parent_size.0 as usize;
            let horizontal_char: char = '─';
            let vertical_char: char = '│';
            let top_left: char = '┌';
            let top_right: char = '┐';
            let bottom_left: char = '└';
            let bottom_right: char = '┘';
            let mut border: String = String::new();

            let color: &str = default_config
                .get("color")
                .and_then(|v| v.as_str())
                .unwrap_or("default");
            let color_prefix: Color = Color::try_from(color).unwrap_or(Color::Reset);

            let mut border_builder: ContentBuilder = ContentBuilder::new();
            border_builder.append_text(
                top_left.to_string()
                    + &horizontal_char.to_string().repeat(width - 2)
                    + top_right.to_string().as_str()
                    + vertical_char.to_string().as_str(),
                color_prefix,
                Color::Reset,
            );

            let mut i = 1;

            let mut rendered_content: Vec<Content> = Vec::new();
            for element in holder.children.iter_mut() {
                rendered_content.push(element.render(
                    page,
                    &(parent_size.0 - 2, parent_size.1 - 2),
                    timer,
                ));
                if let Some(e) = rendered_content.iter().nth_back(1) {
                    rendered_content.last_mut().unwrap().position = Some((
                        e.position.unwrap_or_default().0 + e.size.0 % (parent_size.1 - 2),
                        e.position.unwrap_or_default().1 + e.size.1,
                    ));
                    // let mut bw = BufWriter::new(File::create("debug.log").unwrap());
                    // bw.write_all(format!("{:#?}", rendered_content.last().unwrap().position).as_bytes()).unwrap();
                    // bw.flush().unwrap();
                    
                }
                else {
                    rendered_content.last_mut().unwrap().position = Some((0, 0));
                }
                let pos = rendered_content.last().unwrap().position.unwrap();
                rendered_content
                    .last()
                    .unwrap()
                    .holder
                    .borrow_mut()
                    .position = pos;
                element.position = pos;
                logger::write_log(format!("{:#?}", element.position).as_bytes()).unwrap();
            }

            let mut lines: u16 = 1;

            for c in rendered_content.into_iter() {
                for t in &c.text {
                    let mut temp: String = String::new();
                    for char in t.text.chars() {
                        if (i + 1) % width == 0 {
                            if border_builder
                                .content
                                .last()
                                .unwrap()
                                .text
                                .chars()
                                .last()
                                .unwrap_or(' ')
                                == horizontal_char
                                && should_connect_to_horizontal_chars
                            {
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    t.background_color,
                                );
                                border_builder.append_text(
                                    '┤'.to_string() + vertical_char.to_string().as_str(),
                                    color_prefix,
                                    Color::Reset,
                                );
                                temp = String::new();
                            } else {
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    t.background_color,
                                );
                                border_builder.append_text(
                                    vertical_char.to_string() + vertical_char.to_string().as_str(),
                                    color_prefix,
                                    Color::Reset,
                                );
                                temp = String::new();
                            }
                            i += 2;
                            lines += 1;
                        }
                        if (i - 1) % width == 0
                            && char == horizontal_char
                            && should_connect_to_horizontal_chars
                        {
                            border_builder.content.last_mut().unwrap().text.pop();
                            border_builder.content.last_mut().unwrap().text.push('├');
                        }
                        if char == '\n' {
                            if i % width != 1 {
                                temp.push_str(&*" ".repeat(width - 2 - (i - 1) % width));
                                i += width - 2 - (i - 1) % width;
                                border_builder.append_text(
                                    temp,
                                    t.foreground_color,
                                    t.background_color,
                                );
                                border_builder.append_text(
                                    vertical_char.to_string() + vertical_char.to_string().as_str(),
                                    color_prefix,
                                    Color::Reset,
                                );
                                temp = String::new();
                                i += 2;
                                lines += 1;
                            }
                        } else if char == '\t' {
                            let spaces: usize = 4 - (i - 1) % 4;
                            temp.push_str(&*" ".repeat(spaces));
                            i += spaces;
                        } else {
                            temp.push(char);
                            i += 1;
                        }
                    }
                    if temp != "" {
                        border_builder.append_text(temp, t.foreground_color, t.background_color);
                    }
                }
            }

            if (i - 1) % width == 0 {
                while let Some(last) = border_builder
                    .content
                    .last_mut()
                    .and_then(|c| c.text.chars().last())
                {
                    if last == ' ' {
                        border_builder.content.last_mut().unwrap().text.pop();
                        i -= 1;
                    } else {
                        break;
                    }
                }
                lines -= 1;
            }

            if (i - 1) % width == 0 {
                if border_builder.content.len() > 1 {
                    let ind: usize = border_builder.content.len() - 1;
                    border_builder.content[ind].text.pop();
                } else {
                    border_builder.content.last_mut().unwrap().text.pop();
                }
                i -= 1;
            }

            if !(i % width == 0) {
                border_builder
                    .append_text_default((&*" ".repeat(width - 1 - i % width)).to_string());
                if border_builder.content[border_builder.content.len() - 2]
                    .text
                    .chars()
                    .last()
                    .unwrap_or(' ')
                    == horizontal_char
                    && should_connect_to_horizontal_chars
                {
                    border_builder.append_text('┤'.to_string(), color_prefix, Color::Reset);
                } else {
                    border_builder.append_text(
                        vertical_char.to_string(),
                        color_prefix,
                        Color::Reset,
                    );
                }
            }

            if lines
                < default_config
                    .get("min-height")
                    .unwrap()
                    .as_u64()
                    .unwrap_or(0) as u16
            {
                let additional_lines: u16 = default_config
                    .get("min-height")
                    .unwrap()
                    .as_u64()
                    .unwrap_or(0) as u16
                    - lines;
                for _ in 0..additional_lines {
                    border_builder.append_text(
                        vertical_char.to_string()
                            + &*" ".repeat(width - 2)
                            + vertical_char.to_string().as_str(),
                        color_prefix,
                        Color::Reset,
                    );
                    lines += 1;
                }
            }

            border_builder.append_text(
                bottom_left.to_string()
                    + &horizontal_char.to_string().repeat(width - 2)
                    + bottom_right.to_string().as_str(),
                color_prefix,
                Color::Reset,
            );
            border.push_str(&horizontal_char.to_string().repeat(width - 2));
            border.push(bottom_right);
            border_builder.build(
                true,
                (parent_size.0, lines + 2),
                RefCell::new(holder.to_owned()),
            )
        },
        vec![],
        |args: &Vec<Value>, page: &Page| {
            let res = parse_vec_to_vec(
                (*args
                    .get(0)
                    .unwrap_or(&Value::Array(vec![]))
                    .as_array()
                    .unwrap_or(&vec![]))
                .clone(),
                &page.registry,
            );
            unsafe { std::mem::transmute(res) }
        },
        "border",
        (0, 0),
    );
    e.set_on_hover_func(|holder: &mut Element, _, _, _| {
        if holder.args.len() >= 2 {
            if holder
                .args
                .iter()
                .nth(1)
                .unwrap()
                .as_object()
                .unwrap_or(&Map::new())
                .contains_key("onhover")
            {
                let color: Value = holder
                    .args
                    .get(1)
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("onhover")
                    .unwrap()
                    .as_object()
                    .unwrap()
                    .get("color")
                    .unwrap_or(&json!("default"))
                    .clone();
                holder
                    .args
                    .iter_mut()
                    .nth(1)
                    .unwrap()
                    .as_object_mut()
                    .unwrap()
                    .insert("color".to_string(), color);
            }
        }
    });
    e
});
