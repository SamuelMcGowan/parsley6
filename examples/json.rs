use parsley6::prelude::*;

use parsley6::error::{DefaultError, Error};
use parsley6::stream::CharStream;

type _ParseError<'a> = DefaultError<CharStream<'a>>;

fn main() {}

fn _parse_number<'a>(stream: &mut CharStream<'a>) -> Result<i32, _ParseError<'a>> {
    eat_while_in(Ascii::is_ascii_digit)
        .map_with_span(|s, span| (s, span))
        .and_then(|(s, span): (&str, _)| {
            s.parse::<i32>()
                .map_err(|err| DefaultError::new(Cause::custom(err.to_string()), span))
        })
        .parse(stream)
}
