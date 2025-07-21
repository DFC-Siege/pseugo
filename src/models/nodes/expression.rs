use core::fmt;

use crate::models::nodes::operator::ArithmeticOperator;
use crate::models::nodes::parsing::Parsable;
use color_eyre::eyre::{Result, eyre};

pub enum Expression {
    Variable(String),
    Literal(String),
    BinaryOp {
        left: Box<Expression>,
        operator: Box<ArithmeticOperator>,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Not(Box<Expression>),
}

impl Expression {
    fn parse_not_expression<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let first = parts[0];
        let inner_part = first.trim_start_matches('!');

        if inner_part.is_empty() && parts.len() > 1 {
            let (inner_expr, remaining) = Self::parse(&parts[1..])?;
            Ok((Box::new(Expression::Not(inner_expr)), remaining))
        } else {
            let inner_parts = vec![inner_part];
            let (inner_expr, _) = Self::parse(&inner_parts)?;
            Ok((Box::new(Expression::Not(inner_expr)), &parts[1..]))
        }
    }

    fn parse_function_call<'a>(
        parts: &'a [&'a str],
    ) -> color_eyre::Result<(Box<Self>, &'a [&'a str])> {
        let first = parts[0];
        let paren_pos = first
            .find('(')
            .ok_or_else(|| color_eyre::eyre::eyre!("No opening parenthesis found"))?;
        let name = &first[..paren_pos];

        let mut full_call = String::new();
        let mut paren_count = 0;
        let mut consumed_parts = 0;

        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                full_call.push(' ');
            }
            full_call.push_str(part);
            consumed_parts = i + 1;

            for ch in part.chars() {
                match ch {
                    '(' => paren_count += 1,
                    ')' => paren_count -= 1,
                    _ => {}
                }
            }

            if paren_count == 0 {
                break;
            }
        }

        let start_pos = full_call.find('(').unwrap();
        let end_pos = full_call.rfind(')').unwrap();
        let args_str = &full_call[start_pos + 1..end_pos];
        let args = Self::parse_function_arguments(args_str)?;

        Ok((
            Box::new(Expression::FunctionCall {
                name: name.to_string(),
                args,
            }),
            &parts[consumed_parts..],
        ))
    }

    fn parse_function_arguments(args_str: &str) -> color_eyre::Result<Vec<Expression>> {
        let mut args = Vec::new();
        if !args_str.trim().is_empty() {
            for arg_str in args_str.split(',') {
                let trimmed = arg_str.trim();
                if !trimmed.is_empty() {
                    let arg_parts: Vec<&str> = trimmed.split_whitespace().collect();
                    let (arg_expr, _) = Self::parse(&arg_parts)?;
                    args.push(*arg_expr);
                }
            }
        }
        Ok(args)
    }

    fn parse_simple_expression(token: &str) -> Expression {
        if (token.starts_with('"') && token.ends_with('"'))
            || token.parse::<i32>().is_ok()
            || token.parse::<f64>().is_ok()
        {
            Expression::Literal(token.to_string())
        } else {
            Expression::Variable(token.to_string())
        }
    }

    fn try_parse_binary_operation<'a>(
        parts: &'a [&'a str],
    ) -> Result<Option<(Box<ArithmeticOperator>, &'a [&'a str])>> {
        if parts.len() >= 3 {
            if let Ok((operator, remaining)) = ArithmeticOperator::parse(&parts[1..]) {
                return Ok(Some((operator, remaining)));
            }
        }
        Ok(None)
    }
}

impl Parsable for Expression {
    fn matches(value: &str) -> bool {
        !value.is_empty()
            && (value.chars().all(|c| c.is_alphanumeric() || c == '_')
                || value.parse::<i32>().is_ok()
                || value.parse::<f64>().is_ok()
                || value.starts_with('"') && value.ends_with('"')
                || value.starts_with('!')
                || value.contains('('))
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        if parts.is_empty() {
            return Err(eyre!("No parts to parse for expression"));
        }

        let first = parts[0];

        if first.starts_with('!') {
            return Self::parse_not_expression(parts);
        }

        if first.contains('(') {
            return Self::parse_function_call(parts);
        }

        let simple_expr = Self::parse_simple_expression(first);

        if let Some((operator, remaining)) = Self::try_parse_binary_operation(parts)? {
            let (right_expr, final_remaining) = Self::parse(remaining)?;
            Ok((
                Box::new(Expression::BinaryOp {
                    left: Box::new(simple_expr),
                    operator,
                    right: right_expr,
                }),
                final_remaining,
            ))
        } else {
            Ok((Box::new(simple_expr), &parts[1..]))
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Variable(name) => write!(f, "{name}"),
            Expression::Literal(value) => write!(f, "{value}"),
            Expression::BinaryOp {
                left,
                operator,
                right,
            } => {
                write!(f, "({left} {operator} {right})")
            }
            Expression::FunctionCall { name, args } => {
                write!(f, "{name}(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{arg}")?;
                }
                write!(f, ")")
            }
            Expression::Not(expr) => write!(f, "!{expr}"),
        }
    }
}
