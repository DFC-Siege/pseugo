use crate::models::nodes::parsing::Parsable;
use color_eyre::eyre::{Result, eyre};
use core::fmt;

pub enum ComparisonOperator {
    Eq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
}

pub enum ArithmeticOperator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Modulo,
}

pub enum LogicalOperator {
    And,
    Or,
}

impl Parsable for ComparisonOperator {
    fn matches(value: &str) -> bool {
        matches!(value, "==" | "!=" | ">" | "<" | ">=" | "<=")
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (first, parts) = parts
            .split_first()
            .ok_or(eyre!("No parts to parse for comparison operator"))?;

        let operator = match *first {
            "==" => ComparisonOperator::Eq,
            "!=" => ComparisonOperator::Neq,
            ">" => ComparisonOperator::Gt,
            "<" => ComparisonOperator::Lt,
            ">=" => ComparisonOperator::Gte,
            "<=" => ComparisonOperator::Lte,
            _ => return Err(eyre!("Invalid comparison operator: {}", first)),
        };

        Ok((Box::new(operator), parts))
    }
}

impl Parsable for ArithmeticOperator {
    fn matches(value: &str) -> bool {
        matches!(value, "+" | "-" | "*" | "/" | "%")
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (first, parts) = parts
            .split_first()
            .ok_or(eyre!("No parts to parse for arithmetic operator"))?;

        let operator = match *first {
            "+" => ArithmeticOperator::Addition,
            "-" => ArithmeticOperator::Subtraction,
            "*" => ArithmeticOperator::Multiplication,
            "/" => ArithmeticOperator::Division,
            "%" => ArithmeticOperator::Modulo,
            _ => return Err(eyre!("Invalid arithmetic operator: {}", first)),
        };

        Ok((Box::new(operator), parts))
    }
}

impl Parsable for LogicalOperator {
    fn matches(value: &str) -> bool {
        matches!(value, "&&" | "||")
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (first, parts) = parts
            .split_first()
            .ok_or(eyre!("No parts to parse for logical operator"))?;

        let operator = match *first {
            "&&" => LogicalOperator::And,
            "||" => LogicalOperator::Or,
            _ => return Err(eyre!("Invalid logical operator: {}", first)),
        };

        Ok((Box::new(operator), parts))
    }
}

impl fmt::Display for ComparisonOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            ComparisonOperator::Eq => "==",
            ComparisonOperator::Neq => "!=",
            ComparisonOperator::Gt => ">",
            ComparisonOperator::Lt => "<",
            ComparisonOperator::Gte => ">=",
            ComparisonOperator::Lte => "<=",
        };
        write!(f, "{symbol}")
    }
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            ArithmeticOperator::Addition => "+",
            ArithmeticOperator::Subtraction => "-",
            ArithmeticOperator::Multiplication => "*",
            ArithmeticOperator::Division => "/",
            ArithmeticOperator::Modulo => "%",
        };
        write!(f, "{symbol}")
    }
}

impl fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            LogicalOperator::And => "&&",
            LogicalOperator::Or => "||",
        };
        write!(f, "{symbol}")
    }
}
