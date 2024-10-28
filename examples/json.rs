use parsley6::prelude::*;

use parsley6::error::{DefaultError, Error};
use parsley6::stream::CharStream;

type _ParseError<'a> = DefaultError<CharStream<'a>, ParseErrorCause>;

pub enum ParseErrorCause {
    IntError(std::num::ParseIntError),
}

impl From<std::num::ParseIntError> for ParseErrorCause {
    fn from(err: std::num::ParseIntError) -> Self {
        ParseErrorCause::IntError(err)
    }
}

fn main() {}

fn _parse_number<'a>(stream: &mut CharStream<'a>) -> Result<i32, _ParseError<'a>> {
    eat_while_in(Ascii::is_ascii_digit)
        .with_span()
        .and_then(|(s, span): (&str, _)| {
            s.parse::<i32>()
                .map_err(|err| DefaultError::new(Cause::custom(err), span))
        })
        .parse(stream)
}
