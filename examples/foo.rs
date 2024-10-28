use parsley6::prelude::*;

use parsley6::error::DefaultError;
use parsley6::stream::CharStream;

type Error<'a> = DefaultError<CharStream<'a>>;

fn main() {
    // let mut stream = CharStream::new("bar");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("bat");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("bart");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("bap");
    // let _ = dbg!(bar_bat.parse(&mut stream));

    // let mut stream = CharStream::new("a0a 12");
    // let _ = dbg!(ident.parse(&mut stream));

    // let mut stream = CharStream::new("0a");
    // let _ = dbg!(ident.parse(&mut stream));

    println!("{:?}", test(list, "[bcbcbc]"));
    println!("{:?}", test(list, "[bcbcb]"));
}

fn test<'a, P: Parser<CharStream<'a>, T, Error<'a>>, T>(
    mut parser: P,
    input: &'a str,
) -> Result<T, Error<'a>> {
    let mut stream = CharStream::new(input);
    parser.parse(&mut stream)
}

fn _bar_bat<'a>(stream: &mut CharStream<'a>) -> Result<String, Error<'a>> {
    chain!(
        eat('b'),
        eat('a'),
        eat_in(['r', 't']).with_err_cause(|| "expected `r` or `t`")
    )
    .map_with_slice(|_, slice: &str| slice.to_ascii_uppercase())
    .then_drop(end())
    .with_err_context(|| "while parsing `bar` or `bat`")
    .parse(stream)
}

// fn ident<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
//     chain!(
//         eat_match(|ch: &char| ch.is_ascii_alphabetic() || *ch == '_')
//             .with_err_cause(|| "expected an alphabetic character or `_`"),
//         eat_match(|ch: &char| ch.is_ascii_alphanumeric() || *ch == '_').repeat()
//     )
//     .map_to_slice()
//     .parse(stream)
// }

fn _ident<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
    chain!(
        eat_in(|ch: &char| ch.is_ascii_alphabetic() || *ch == '_')
            .with_err_cause(|| "expected an alphabetic character or `_`"),
        eat_while_in(|ch: &char| ch.is_ascii_alphanumeric() || *ch == '_')
    )
    .map_to_slice()
    .parse(stream)
}

fn list<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
    between(
        eat('['),
        chain!(eat('b'), eat('c'))
            .repeat_until(|ch: &char| *ch == ']')
            .map_to_slice(),
        eat(']'),
    )
    .parse(stream)
}

fn abcdef<'a>(stream: &mut CharStream<'a>) -> Result<&'a str, Error<'a>> {
    select!(
        'a' => chain!(eat('a'), eat('b'), eat('c')),
        'd' => chain!(eat('d'), eat('e'), eat('f')),
    )
    .map_to_slice()
    .parse(stream)
}
