use std::{marker::PhantomData, slice::Iter, str::Chars};

pub trait Stream<'a> {
    type Token: Clone + PartialEq;
    type Slice: Slice + PartialEq + ?Sized;

    fn peek_token(&self) -> Option<Self::Token>;

    #[must_use = "If you don't need the token, use `Input::advance`"]
    fn next_token(&mut self) -> Option<Self::Token>;

    fn as_slice(&self) -> &'a Self::Slice;

    #[inline]
    fn advance(&mut self) {
        let _ = self.next_token();
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.peek_token().is_none()
    }
}

/// A slice, for example `[T]` or [`str`].
pub trait Slice {
    fn try_prefix(&self, len: usize) -> Option<&Self>;

    #[inline]
    fn prefix(&self, len: usize) -> &Self {
        self.try_prefix(len).expect("len out of range")
    }
}

impl Slice for str {
    #[inline]
    fn try_prefix(&self, len: usize) -> Option<&Self> {
        self.get(..len)
    }
}

impl<T> Slice for [T] {
    #[inline]
    fn try_prefix(&self, len: usize) -> Option<&Self> {
        self.get(..len)
    }
}

impl<'a> Stream<'a> for Chars<'a> {
    type Token = char;
    type Slice = str;

    #[inline]
    fn as_slice(&self) -> &'a Self::Slice {
        self.as_str()
    }

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.next()
    }
}

impl<'a, T: Clone + PartialEq> Stream<'a> for Iter<'a, T> {
    type Token = &'a T;
    type Slice = [T];

    #[inline]
    fn as_slice(&self) -> &'a Self::Slice {
        self.as_slice()
    }

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.next()
    }
}

pub struct StreamWithState<'a, S: Stream<'a>, State> {
    pub stream: S,
    pub state: State,
    _phantom: PhantomData<&'a ()>,
}

impl<'a, S: Stream<'a>, State> StreamWithState<'a, S, State> {
    #[inline]
    pub fn new(stream: S, state: State) -> Self {
        Self {
            stream,
            state,
            _phantom: PhantomData,
        }
    }
}

impl<'a, S: Stream<'a>, State> Stream<'a> for StreamWithState<'a, S, State> {
    type Token = S::Token;
    type Slice = S::Slice;

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.stream.peek_token()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.stream.next_token()
    }

    #[inline]
    fn as_slice(&self) -> &'a Self::Slice {
        self.stream.as_slice()
    }

    #[inline]
    fn advance(&mut self) {
        self.stream.advance();
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.stream.at_end()
    }
}
