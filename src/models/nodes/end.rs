use color_eyre::eyre::{Result, eyre};

use crate::{
    indent_writeln,
    models::nodes::{formatter::IndentFormatter, parsing::Parsable},
};

pub struct End;
impl End {
    const END: &'static str = "end";
}

impl IndentFormatter for End {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> core::fmt::Result {
        indent_writeln!(f, indent_count - 1, "{}", Self::END);
        Ok(())
    }
}

impl Parsable for End {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == Self::END
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;
        if !Self::matches(start) {
            return Err(eyre!("next element is not {}", Self::END));
        }

        Ok((Box::new(Self {}), parts))
    }
}
