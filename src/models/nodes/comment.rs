use color_eyre::eyre::{Result, eyre};

use crate::{
    indent_writeln,
    models::nodes::{formatter::IndentFormatter, parsing::Parsable},
};

pub struct Comment {
    value: String,
}

impl Comment {
    const KEYWORD: &'static str = "--";
}

impl IndentFormatter for Comment {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> color_eyre::Result<usize> {
        indent_writeln!(f, indent_count, "// {}", self.value)
    }
}

impl Parsable for Comment {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == Comment::KEYWORD
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;
        if !Self::matches(start) {
            return Err(eyre!("next element is not {}", Comment::KEYWORD));
        }

        let split_pos = parts
            .iter()
            .position(|part| Self::matches(part))
            .unwrap_or(parts.len());

        let (word_parts, parts) = parts.split_at(split_pos);
        let parts = if split_pos < parts.len() {
            &parts[1..]
        } else {
            parts
        };

        Ok((
            Box::new(Self {
                value: word_parts.join(" "),
            }),
            parts,
        ))
    }
}
