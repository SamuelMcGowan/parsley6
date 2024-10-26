use parsley6::prelude::*;

use parsley6::error::DefaultError;
use parsley6::stream::CharStream;

fn main() {
    let mut stream = CharStream::new("hello world");
    let _ = dbg!(five_letters.parse(&mut stream));
}

fn five_letters<'a>(
    stream: &mut CharStream<'a>,
) -> Result<(char, char, char, char, char), DefaultError<CharStream<'a>>> {
    let a = eat_match(Ascii::is_ascii_alphabetic);
    chain((a, a, a, a, eat('o'))).then_drop(end()).parse(stream)
}
