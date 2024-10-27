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
    pub use crate::token::{eat, eat_any, eat_match, end, peek, peek_any, peek_match};

    pub use crate::combinator::chain::{between, prefixed, suffixed};
    pub use crate::{alt, chain};
}
