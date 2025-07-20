use crate::{
    indent_writeln,
    models::nodes::{expression::Expression, formatter::IndentFormatter, parsing::Parsable},
};
use color_eyre::eyre::{Result, eyre};

pub struct Assignment {
    var: String,
    value: Expression,
}

impl Assignment {
    const KEYWORD: &'static str = "assign";
}

impl IndentFormatter for Assignment {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> color_eyre::Result<usize> {
        indent_writeln!(f, indent_count, "{} = {}", self.var, self.value)
    }
}

impl Parsable for Assignment {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == Self::KEYWORD
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        if parts.len() < 4 {
            return Err(eyre!(
                "Assignment requires at least 4 parts: {} var = value",
                Self::KEYWORD
            ));
        }

        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;

        if !Self::matches(start) {
            return Err(eyre!("first element is not {}", Self::KEYWORD));
        }

        if parts.len() < 3 {
            return Err(eyre!(
                "Assignment missing parts: expected 'var = expression'"
            ));
        }

        let var = parts[0].to_string();

        if parts[1] != "=" {
            return Err(eyre!(
                "Expected '=' after variable name, found '{}'",
                parts[1]
            ));
        }

        let (expression, parts) = Expression::parse(&parts[2..])?;

        Ok((
            Box::new(Self {
                var,
                value: *expression,
            }),
            parts,
        ))
    }
}
