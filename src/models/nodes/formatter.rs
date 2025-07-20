pub trait IndentFormatter {
    fn fmt_indent(&self, f: &mut core::fmt::Formatter<'_>, indent_count: usize) -> usize;
}

#[macro_export]
macro_rules! indent_write {
    ($f:expr, $indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            write!($f, "{}{}", spaces, format_args!($($arg)*));
            $indent
        }
    };
}

#[macro_export]
macro_rules! indent_writeln {
    ($f:expr, $indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            writeln!($f, "{}{}", spaces, format_args!($($arg)*));
            $indent
        }
    };
}

#[macro_export]
macro_rules! indent_format {
    ($indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            format!("{}{}", spaces, format_args!($($arg)*));
            $indent
        }
    };
}

#[macro_export]
macro_rules! indent_format_args {
    ($indent:expr, $($arg:tt)*) => {
        {
            const TAB_SIZE: usize = 8;
            let spaces = " ".repeat(TAB_SIZE * $indent);
            format_args!("{}{}", spaces, format_args!($($arg)*));
            $indent
        }
    };
}
