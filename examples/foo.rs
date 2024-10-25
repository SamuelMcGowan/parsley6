use std::str::Chars;

use parsley6::{
    error::BuiltinError,
    parser::Parser,
    token::{eat_match, text::Ascii},
};

fn main() {
    let mut stream = "hello world".chars();
    let _ = dbg!(five_letters.parse(&mut stream));
}

fn five_letters<'a>(
    stream: &mut Chars<'a>,
) -> Result<(char, char, char, char, char), BuiltinError<'a, Chars<'a>>> {
    let a = eat_match(Ascii::is_ascii_alphabetic);
    (a, a, a, a, a).parse(stream)
}
