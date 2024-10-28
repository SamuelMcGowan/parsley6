pub mod error;
pub mod parser;
pub mod stream;

pub mod combinator;
pub mod token;
pub mod token_set;

mod sealed {
    pub trait Sealed {}
}

pub mod prelude {
    pub use crate::parser::Parser;

    pub use crate::token_set::TokenSet;

    pub use crate::token::text::Ascii;
    pub use crate::token::{eat, eat_in, eat_while_in, end, peek, peek_in};

    pub use crate::combinator::chain::{between, prefixed, suffixed};
    pub use crate::{alt, chain};
}
