use color_eyre::eyre::{Result, eyre};

use crate::{
    indent_writeln,
    models::nodes::{
        condition::Condition, formatter::IndentFormatter, node::Node, parsing::Parsable,
    },
};

pub enum LoopType {
    While(Condition),
    For { item: String, list: String },
}

impl LoopType {
    const FOR: &'static str = "for";
    const WHILE: &'static str = "while";
}

pub struct Loop {
    loop_type: LoopType,
    body: Vec<Node>,
}

impl IndentFormatter for Loop {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        mut indent_count: usize,
    ) -> color_eyre::Result<usize> {
        match &self.loop_type {
            LoopType::While(condition) => {
                indent_count = indent_writeln!(f, indent_count, "while {condition}")?;
            }
            LoopType::For { item, list } => {
                indent_count = indent_writeln!(f, indent_count, "for {item} in {list}")?;
            }
        }

        indent_count += 1;
        for n in &self.body {
            indent_count = n.fmt_indent(f, indent_count)?;
        }
        Ok(indent_count)
    }
}

impl Parsable for Loop {
    fn matches(value: &str) -> bool {
        let value = value.to_lowercase();
        value == LoopType::FOR || value == LoopType::WHILE
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (val, parts) = parts.split_first().ok_or(eyre!("can't get next element"))?;

        match *val {
            LoopType::FOR => {
                let (item, parts) = parts.split_first().ok_or(eyre!("can't get next element"))?;
                let (list, parts) = parts.split_first().ok_or(eyre!("can't get next element"))?;
                let (body, parts) = Node::build_from_parts(parts)?;
                Ok((
                    Box::new(Self {
                        loop_type: LoopType::For {
                            item: item.to_string(),
                            list: list.to_string(),
                        },
                        body,
                    }),
                    parts,
                ))
            }
            LoopType::WHILE => {
                let (condition, parts) = Condition::parse(parts)?;
                let (body, parts) = Node::build_from_parts(parts)?;

                Ok((
                    Box::new(Self {
                        loop_type: LoopType::While(*condition),
                        body,
                    }),
                    parts,
                ))
            }
            _ => Err(eyre!(
                "next element is not {} or {}",
                LoopType::FOR,
                LoopType::WHILE
            )),
        }
    }
}
