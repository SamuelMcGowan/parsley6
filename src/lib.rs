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
    pub use crate::token::{consume, seek_past, seek_until};
    pub use crate::token::{eat, peek};
    pub use crate::token::{eat_slice, end, peek_slice};

    pub use crate::combinator::chain::{between, prefixed, suffixed};
    pub use crate::{chain, select};
}
