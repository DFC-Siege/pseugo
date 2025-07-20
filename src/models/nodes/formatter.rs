pub trait IndentFormatter {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> color_eyre::Result<usize>;
}

pub const TAB_SIZE: usize = 8;

#[macro_export]
macro_rules! indent_write {
    ($f:expr, $indent:expr, $($arg:tt)*) => {
        {
            write!($f, $($arg)*)
                .map(|_| $indent)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to write indented text: {}", e))
        }
    };
}

#[macro_export]
macro_rules! indent_writeln {
    ($f:expr, $indent:expr, $($arg:tt)*) => {
        {
            let spaces = " ".repeat($crate::models::nodes::formatter::TAB_SIZE * $indent);
            write!($f, "{}", spaces)?;
            write!($f, $($arg)*)
                .map(|_| $indent)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to write indented line: {}", e))
        }
    };
}
