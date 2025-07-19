use color_eyre::eyre::Result;

pub trait Parsable {
    fn matches(value: &str) -> bool;
    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])>;
}
