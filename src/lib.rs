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
    pub use crate::token::{eat_if, eat_while, peek_if};

    pub use crate::combinator::chain::{between, prefixed, suffixed};
    pub use crate::{chain, select};
}
