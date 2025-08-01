use crate::{
    indent_writeln,
    models::nodes::{formatter::IndentFormatter, node::Node, parsing::Parsable},
};
use color_eyre::eyre::{Result, eyre};

pub struct Start {
    body: Vec<Node>,
}

impl Start {
    const KEYWORD: &'static str = "start";
}

impl IndentFormatter for Start {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        mut indent_count: usize,
    ) -> color_eyre::Result<usize> {
        indent_count = indent_writeln!(f, indent_count, "start")?;

        for n in &self.body {
            indent_count = n.fmt_indent(f, indent_count)?;
        }
        Ok(indent_count)
    }
}

impl Parsable for Start {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == Self::KEYWORD
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (start, parts) = parts
            .split_first()
            .ok_or_else(|| eyre!("Cannot parse {}: input is empty", Self::KEYWORD))?;

        if !Start::matches(start) {
            return Err(eyre!("Expected '{}' but found '{}'", Self::KEYWORD, start));
        }

        let (body, parts) = Node::build_from_parts(parts)?;

        Ok((Box::new(Self { body }), parts))
    }
}
