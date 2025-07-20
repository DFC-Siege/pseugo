pub trait IndentFormatter {
    fn fmt_indent(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        indent_count: usize,
    ) -> color_eyre::Result<usize>;
}

#[macro_export]
macro_rules! indent_write {
    ($f:expr, $indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            write!($f, "{}{}", spaces, format_args!($($arg)*))
                .map(|_| $indent)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to write indented text: {}", e))
        }
    };
}

#[macro_export]
macro_rules! indent_writeln {
    ($f:expr, $indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            writeln!($f, "{}{}", spaces, format_args!($($arg)*))
                .map(|_| $indent)
                .map_err(|e| color_eyre::eyre::eyre!("Failed to write indented line: {}", e))
        }
    };
}

#[macro_export]
macro_rules! indent_format {
    ($indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            Ok::<usize, color_eyre::eyre::Error>({
                format!("{}{}", spaces, format_args!($($arg)*));
                $indent
            })
        }
    };
}

#[macro_export]
macro_rules! indent_format_args {
    ($indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            Ok::<(std::fmt::Arguments<'_>, usize), color_eyre::eyre::Error>((
                format_args!("{}{}", spaces, format_args!($($arg)*)),
                $indent
            ))
        }
    };
}
