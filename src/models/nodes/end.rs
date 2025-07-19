use core::fmt;
use std::fmt::Formatter;

use color_eyre::eyre::{Result, eyre};

use crate::models::nodes::parsing::Parsable;

pub struct End;
impl End {
    const END: &'static str = "end";
}

impl fmt::Display for End {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "{}", Self::END)
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
