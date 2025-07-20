use core::fmt;

use color_eyre::eyre::{Result, eyre};

use crate::{
    indent_writeln,
    models::nodes::{
        assignment::Assignment,
        comment::Comment,
        condition::{Else, ElseIf, If},
        end::End,
        formatter::IndentFormatter,
        function::FunctionCall,
        loops::Loop,
        parsing::Parsable,
        return_node::Return,
        start::Start,
    },
};

pub struct Break;
impl Parsable for Break {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == "loop"
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;
        if !Loop::matches(start) {
            return Err(eyre!("first element is not loop"));
        }

        Ok((Box::new(Self {}), parts))
    }
}

impl IndentFormatter for Break {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> core::fmt::Result {
        indent_writeln!(f, indent_count, "break");
        Ok(())
    }
}

pub struct Continue;
impl Parsable for Continue {
    fn matches(value: &str) -> bool {
        value.to_lowercase() == "loop"
    }

    fn parse<'a>(parts: &'a [&'a str]) -> Result<(Box<Self>, &'a [&'a str])> {
        let (start, parts) = parts
            .split_first()
            .ok_or(eyre!("can't get first element"))?;
        if !Loop::matches(start) {
            return Err(eyre!("first element is not loop"));
        }

        Ok((Box::new(Self {}), parts))
    }
}

impl IndentFormatter for Continue {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> core::fmt::Result {
        indent_writeln!(f, indent_count, "continue");
        Ok(())
    }
}

pub enum Node {
    Start(Start),
    Loop(Loop),
    Break(Break),
    Continue(Continue),
    If(If),
    ElseIf(ElseIf),
    Else(Else),
    End(End),
    Assignment(Assignment),
    FunctionCall(FunctionCall),
    Return(Return),
    Comment(Comment),
}

impl Node {
    pub fn new(input: &str) -> Result<Self> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let (start, _) = Start::parse(&parts)?;
        let start = Self::Start(*start);

        Ok(start)
    }

    pub fn build_from_parts<'a>(mut parts: &'a [&'a str]) -> Result<(Vec<Self>, &'a [&'a str])> {
        let mut nodes = Vec::new();

        while !parts.is_empty() {
            let first = parts.first().ok_or(eyre!("unable to get first in parts"))?;

            if Start::matches(first) {
                return Err(eyre!("Start can only be used once"));
            }

            macro_rules! try_parse {
                ($type:ty, $variant:ident) => {
                    if <$type>::matches(first) {
                        let (node, p) = <$type>::parse(parts)?;
                        parts = p;
                        nodes.push(Node::$variant(*node));
                        continue;
                    }
                };
            }

            try_parse!(Loop, Loop);
            try_parse!(Break, Break);
            try_parse!(Continue, Continue);
            try_parse!(If, If);
            try_parse!(ElseIf, ElseIf);
            try_parse!(Else, Else);
            try_parse!(End, End);
            try_parse!(Assignment, Assignment);
            try_parse!(FunctionCall, FunctionCall);
            try_parse!(Return, Return);
            try_parse!(Comment, Comment);

            // panic!("Parts: {parts:?}");
            return Err(eyre!("Unknown node type: {first}"));
        }

        Ok((nodes, parts))
    }
}

impl IndentFormatter for Node {
    fn fmt_indent(&self, f: &mut fmt::Formatter<'_>, indent_count: usize) -> fmt::Result {
        match self {
            Node::Start(val) => val.fmt_indent(f, indent_count),
            Node::Loop(val) => val.fmt_indent(f, indent_count),
            Node::Break(val) => val.fmt_indent(f, indent_count),
            Node::Continue(val) => val.fmt_indent(f, indent_count),
            Node::If(val) => val.fmt_indent(f, indent_count),
            Node::ElseIf(val) => val.fmt_indent(f, indent_count),
            Node::Else(val) => val.fmt_indent(f, indent_count),
            Node::Assignment(val) => val.fmt_indent(f, indent_count),
            Node::FunctionCall(val) => val.fmt_indent(f, indent_count),
            Node::Return(val) => val.fmt_indent(f, indent_count),
            Node::Comment(val) => val.fmt_indent(f, indent_count),
            Node::End(val) => val.fmt_indent(f, indent_count),
        }?;

        Ok(())
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt_indent(f, 0)
    }
}
