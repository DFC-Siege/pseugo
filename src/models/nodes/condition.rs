use core::fmt;

use color_eyre::eyre::{Result, eyre};

use crate::{
    indent_writeln,
    models::nodes::{
        expression::Expression,
        formatter::IndentFormatter,
        node::Node,
        operator::{ComparisonOperator, LogicalOperator},
        parsing::Parsable,
    },
};

pub struct If {
    condition: Condition,
    body: Vec<Node>,
}

impl IndentFormatter for If {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> core::fmt::Result {
        indent_writeln!(f, indent_count, "if {}", self.condition);
        for node in &self.body {
            node.fmt_indent(f, indent_count + 1)?;
        }

        Ok(())
    }
}

impl Parsable for If {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == "if"
    }
    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        if parts.is_empty() {
            return Err(eyre!("No parts to parse for if statement"));
        }

        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;

        if !Self::matches(start) {
            return Err(eyre!("first element is not if"));
        }

        let (condition, parts) = Condition::parse(parts)?;
        let (body, parts) = Node::build_from_parts(parts)?;

        Ok((
            Box::new(Self {
                condition: *condition,
                body,
            }),
            parts,
        ))
    }
}

pub struct ElseIf {
    condition: Condition,
    body: Vec<Node>,
}

impl IndentFormatter for ElseIf {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> core::fmt::Result {
        indent_writeln!(f, indent_count - 1, "else if {}", self.condition);

        for node in &self.body {
            node.fmt_indent(f, indent_count)?;
        }

        Ok(())
    }
}

impl Parsable for ElseIf {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == "elseif"
    }
    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        if parts.is_empty() {
            return Err(eyre!("No parts to parse for elseif statement"));
        }

        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;

        if !Self::matches(start) {
            return Err(eyre!("first element is not elseif"));
        }

        let (condition, parts) = Condition::parse(parts)?;
        let (body, parts) = Node::build_from_parts(parts)?;

        Ok((
            Box::new(Self {
                condition: *condition,
                body,
            }),
            parts,
        ))
    }
}

pub struct Else {
    body: Vec<Node>,
}

impl IndentFormatter for Else {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> core::fmt::Result {
        indent_writeln!(f, indent_count - 1, "else");

        for node in &self.body {
            node.fmt_indent(f, indent_count)?;
        }

        Ok(())
    }
}

impl Parsable for Else {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == "else"
    }
    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        if parts.is_empty() {
            return Err(eyre!("No parts to parse for else statement"));
        }

        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;

        if !Self::matches(start) {
            return Err(eyre!("first element is not else"));
        }
        let (body, parts) = Node::build_from_parts(parts)?;

        Ok((Box::new(Self { body }), parts))
    }
}

pub enum Condition {
    ComparisonOperator {
        left: Box<Expression>,
        operator: Box<ComparisonOperator>,
        right: Box<Expression>,
    },
    LogicalOperator {
        left: Box<Condition>,
        operator: Box<LogicalOperator>,
        right: Box<Condition>,
    },
    Value(String),
    NotValue(String),
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Condition::ComparisonOperator {
                left,
                operator,
                right,
            } => {
                write!(f, "({left} {operator} {right})")
            }
            Condition::LogicalOperator {
                left,
                operator,
                right,
            } => {
                write!(f, "({left} {operator} {right})")
            }
            Condition::Value(value) => write!(f, "{value}"),
            Condition::NotValue(value) => write!(f, "!{value}"),
        }
    }
}

impl Parsable for Condition {
    fn matches(value: &str) -> bool {
        Expression::matches(value) || value.starts_with('!')
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        if parts.is_empty() {
            return Err(eyre!("No parts to parse for condition"));
        }

        if parts[0].starts_with('!') {
            let value = parts[0].trim_start_matches('!');
            return Ok((
                Box::new(Condition::NotValue(value.to_string())),
                &parts[1..],
            ));
        }

        let (left_expr, parts) = Expression::parse(parts)?;

        if parts.len() >= 2 {
            if let Ok((comp_operator, parts2)) = ComparisonOperator::parse(parts) {
                let (right_expr, after_comparison) = Expression::parse(parts2)?;

                let comparison_condition = Condition::ComparisonOperator {
                    left: left_expr,
                    operator: comp_operator,
                    right: right_expr,
                };

                if after_comparison.len() >= 2 {
                    if let Ok((logical_op, parts3)) = LogicalOperator::parse(after_comparison) {
                        let (right_condition, parts) = Condition::parse(parts3)?;
                        return Ok((
                            Box::new(Condition::LogicalOperator {
                                left: Box::new(comparison_condition),
                                operator: logical_op,
                                right: right_condition,
                            }),
                            parts,
                        ));
                    }
                }

                return Ok((Box::new(comparison_condition), after_comparison));
            }
        }

        if let Expression::Variable(var_name) = left_expr.as_ref() {
            let simple_condition = Condition::Value(var_name.clone());

            if parts.len() >= 2 {
                if let Ok((logical_op, parts2)) = LogicalOperator::parse(parts) {
                    let (right_condition, parts) = Condition::parse(parts2)?;
                    return Ok((
                        Box::new(Condition::LogicalOperator {
                            left: Box::new(simple_condition),
                            operator: logical_op,
                            right: right_condition,
                        }),
                        parts,
                    ));
                }
            }

            return Ok((Box::new(simple_condition), parts));
        }

        Err(eyre!("Unable to parse condition from remaining parts"))
    }
}
