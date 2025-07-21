use crate::{
    indent_write, indent_writeln,
    models::nodes::{expression::Expression, formatter::IndentFormatter, parsing::Parsable},
};
use color_eyre::eyre::{Result, eyre};

pub struct FunctionCall {
    name: String,
    args: Vec<Expression>,
}

impl IndentFormatter for FunctionCall {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> color_eyre::Result<usize> {
        indent_writeln!(f, indent_count, "{} ", self.name)?;
        for (i, arg) in self.args.iter().enumerate() {
            if i > 0 {
                indent_write!(f, indent_count, ", ")?;
            }
            indent_write!(f, indent_count, "{arg}")?;
        }
        indent_writeln!(f, indent_count, ":")
    }
}

impl Parsable for FunctionCall {
    fn matches(value: &str) -> bool {
        value.contains("(")
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        if parts.len() < 3 {
            return Err(eyre!("Function call requires at least 2 parts"));
        }

        let (next, parts) = parts.split_first().ok_or(eyre!("can't get next element"))?;

        if !Self::matches(next) {
            return Err(eyre!("next element is not function"));
        }

        let (name, mut parts) = parts
            .split_first()
            .ok_or(eyre!("can't get function name"))?;

        let mut args = Vec::new();

        while !parts.is_empty() {
            println!("{}", parts.len());
            let (arg, p) = Expression::parse(parts)?;
            args.push(*arg);
            parts = p;
        }

        Ok((
            Box::new(Self {
                name: name.to_string(),
                args,
            }),
            parts,
        ))
    }
}
