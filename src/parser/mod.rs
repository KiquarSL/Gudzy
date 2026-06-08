pub mod expr;
pub mod ast;
mod parser;

pub use parser::Parser;

#[derive(Debug, Clone)]
pub struct Info {
    pub line: usize,
    pub offset: usize,
    pub len: usize,
}

#[macro_export]
macro_rules! info {
    ($line:expr, $offset:expr, $len:expr) => {
        $crate::parser::Info {
            line: $line,
            offset: $offset,
            len: $len,
        }
    };
    ($tkn:expr) => {
        info!($tkn.line, $tkn.offset, $tkn.len)
    };
}
