use std::marker::PhantomData;

use derive_where::derive_where;

use crate::error::{Error, ErrorCause};
use crate::parser::Parser;
use crate::stream::Stream;

pub trait AltParsers<S, O, E>
where
    S: Stream,
    E: Error<S>,
{
    fn parse_alt(&mut self, stream: &mut S) -> Result<O, E>;
}

macro_rules! impl_alt_parsers {
    ($($parser:ident $n:tt),+) => {
        impl<S, Out, Err, $($parser,)*>
        AltParsers<S, Out, Err> for ($($parser,)*)
        where
            S: Stream,
            Err: Error<S>,
            $($parser: Parser<S, Out, Err>,)*
        {
            #[allow(non_snake_case)]
            #[inline]
            fn parse_alt(&mut self, stream: &mut S) -> Result<Out, Err>
            {
                $(
                    if let Ok(output) = self.$n.parse(stream) {
                        return Ok(output);
                    }
                )*

                Err(Err::new(Err::Cause::unknown(), stream.peek_token_span()))
            }
        }
    };
}

impl_alt_parsers! { A 0 }
impl_alt_parsers! { A 0, B 1 }
impl_alt_parsers! { A 0, B 1, C 2 }
impl_alt_parsers! { A 0, B 1, C 2, D 3 }
impl_alt_parsers! { A 0, B 1, C 2, D 3, E 4 }
impl_alt_parsers! { A 0, B 1, C 2, D 3, E 4, F 5 }
impl_alt_parsers! { A 0, B 1, C 2, D 3, E 4, F 5, G 6 }
impl_alt_parsers! { A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7 }

#[macro_export]
macro_rules! alt {
    ($($e:expr),+ $(,)?) => {
        $crate::combinator::alt::alt_inner(($($e),*))
    };
}

#[inline]
#[doc(hidden)]
pub fn alt_inner<Parsers, S, O, E>(parsers: Parsers) -> Alt<Parsers, S, O, E>
where
    Parsers: AltParsers<S, O, E>,
    S: Stream,
    E: Error<S>,
{
    Alt {
        parsers,
        _phantom: PhantomData,
    }
}

#[derive_where(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash; Parsers)]
pub struct Alt<Parsers, S, O, E> {
    parsers: Parsers,
    _phantom: PhantomData<*const (S, O, E)>,
}

impl<Parsers, S, O, E> Parser<S, O, E> for Alt<Parsers, S, O, E>
where
    Parsers: AltParsers<S, O, E>,
    S: Stream,
    E: Error<S>,
{
    fn parse(&mut self, stream: &mut S) -> Result<O, E> {
        self.parsers.parse_alt(stream)
    }
}
