use color_eyre::eyre::{Result, eyre};

use crate::{
    indent_writeln,
    models::nodes::{formatter::IndentFormatter, parsing::Parsable},
};

pub struct Comment {
    value: String,
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
        value.to_lowercase() == "//"
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;
        if !Self::matches(start) {
            return Err(eyre!("first element is not loop"));
        }

        Ok((Box::new(Self { value: "".into() }), parts))
    }
}
