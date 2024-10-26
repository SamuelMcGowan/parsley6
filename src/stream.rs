use std::{marker::PhantomData, slice::Iter, str::Chars};

pub trait Stream<'a> {
    type Token: Clone + PartialEq;
    type Slice: PartialEq + ?Sized + 'a;

    fn peek_token(&self) -> Option<Self::Token>;

    #[must_use = "If you don't need the token, use `Input::advance`"]
    fn next_token(&mut self) -> Option<Self::Token>;

    fn peek_slice(&self, len: usize) -> Option<&'a Self::Slice>;

    #[must_use = "If you don't need the slice, use `Input::advance_len`"]
    fn next_slice(&mut self, len: usize) -> Option<&'a Self::Slice>;

    #[inline]
    fn advance(&mut self) {
        let _ = self.next_token();
    }

    fn advance_len(&mut self, len: usize) {
        let _ = self.next_slice(len);
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.peek_token().is_none()
    }
}

impl<'a> Stream<'a> for Chars<'a> {
    type Token = char;
    type Slice = str;

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.next()
    }

    #[inline]
    fn peek_slice(&self, len: usize) -> Option<&'a Self::Slice> {
        self.as_str().split_at_checked(len).map(|(slice, _)| slice)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<&'a Self::Slice> {
        let (slice, rest) = self.as_str().split_at_checked(len)?;
        *self = rest.chars();
        Some(slice)
    }
}

impl<'a, T: Clone + PartialEq> Stream<'a> for Iter<'a, T> {
    type Token = &'a T;
    type Slice = [T];

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.next()
    }

    #[inline]
    fn peek_slice(&self, len: usize) -> Option<&'a Self::Slice> {
        self.as_slice()
            .split_at_checked(len)
            .map(|(slice, _)| slice)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<&'a Self::Slice> {
        let (slice, rest) = self.as_slice().split_at_checked(len)?;
        *self = rest.iter();
        Some(slice)
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
    fn peek_slice(&self, len: usize) -> Option<&'a Self::Slice> {
        self.stream.peek_slice(len)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<&'a Self::Slice> {
        self.stream.next_slice(len)
    }

    #[inline]
    fn advance(&mut self) {
        self.stream.advance();
    }

    #[inline]
    fn advance_len(&mut self, len: usize) {
        self.stream.advance_len(len);
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.stream.at_end()
    }
}
