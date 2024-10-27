use parsley6::prelude::*;

use parsley6::error::DefaultError;
use parsley6::stream::CharStream;

fn main() {
    let mut stream = CharStream::new("bar");
    let _ = dbg!(bar_bat.parse(&mut stream));

    let mut stream = CharStream::new("bat");
    let _ = dbg!(bar_bat.parse(&mut stream));

    let mut stream = CharStream::new("bart");
    let _ = dbg!(bar_bat.parse(&mut stream));

    let mut stream = CharStream::new("bap");
    let _ = dbg!(bar_bat.parse(&mut stream));
}

fn bar_bat<'a>(
    stream: &mut CharStream<'a>,
) -> Result<(char, char, char), DefaultError<CharStream<'a>>> {
    chain!(
        eat('b'),
        eat('a'),
        alt!(eat('r'), eat('t')).with_err_cause(|| "expected `r` or `t`")
    )
    .then_drop(end())
    .with_context(|| "while parsing `bar` or `bat`")
    .parse(stream)
}
