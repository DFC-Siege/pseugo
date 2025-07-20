use crate::{
    indent_writeln,
    models::nodes::{expression::Expression, formatter::IndentFormatter, parsing::Parsable},
};
use color_eyre::eyre::{Ok, Result, eyre};

pub struct Return {
    value: Expression,
}

impl Return {
    const KEYWORD: &'static str = "return";
}

impl IndentFormatter for Return {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> color_eyre::Result<usize> {
        indent_writeln!(f, indent_count, "{} {}", Self::KEYWORD, self.value)
    }
}

impl Parsable for Return {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == Self::KEYWORD
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (first, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;
        if !Self::matches(first) {
            return Err(eyre!("first element is not {}", Self::KEYWORD));
        }
        let (expression, parts) = Expression::parse(parts)?;
        Ok((Box::new(Self { value: *expression }), parts))
    }
}
