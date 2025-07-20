pub trait IndentFormatter {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> color_eyre::Result<usize>;
}

const INDENT_COLORS: &[&str] = &[
    "\x1b[36m", // Cyan
    "\x1b[35m", // Magenta
    "\x1b[33m", // Yellow
    "\x1b[32m", // Green
    "\x1b[34m", // Blue
    "\x1b[31m", // Red
];

pub const RESET_COLOR: &str = "\x1b[0m";
pub const TAB_SIZE: usize = 8;

pub fn get_indent_color(indent_level: usize) -> &'static str {
    INDENT_COLORS[indent_level % INDENT_COLORS.len()]
}

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
            let color = $crate::models::nodes::formatter::get_indent_color($indent);
            let reset = $crate::models::nodes::formatter::RESET_COLOR;
            write!($f, "{}", color)?;
            write!($f, "{}", spaces)?;
            write!($f, $($arg)*)?;
            writeln!($f, "{}", reset)
                .map(|_| $indent)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to write indented line: {}", e))
        }
    };
}
