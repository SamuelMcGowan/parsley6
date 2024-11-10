pub mod error;
pub mod parser;
pub mod stream;

pub mod combinator;
pub mod token;

mod sealed {
    pub trait Sealed {}
}

pub mod prelude {
    pub use crate::parser::Parser;

    pub use crate::token::text::Ascii;
    pub use crate::token::{eat, eat_slice, end, peek, peek_slice};
    pub use crate::token::{eat_if, peek_if};
    pub use crate::token::{eat_while, seek};

    pub use crate::combinator::{between, prefixed, suffixed};
    pub use crate::{chain, select};
}
