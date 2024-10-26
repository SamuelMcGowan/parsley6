use std::{marker::PhantomData, ops::Range, slice::Iter, str::Chars};

pub trait Stream<'a> {
    type Token: Clone + PartialEq;
    type Slice: PartialEq + ?Sized + 'a;

    type SourceLoc: Default + Clone;

    fn peek_token(&self) -> Option<Self::Token>;

    #[must_use = "If you don't need the token, use `Input::advance`"]
    fn next_token(&mut self) -> Option<Self::Token>;

    fn peek_slice(&self, len: usize) -> Option<&'a Self::Slice>;

    #[must_use = "If you don't need the slice, use `Input::advance_len`"]
    fn next_slice(&mut self, len: usize) -> Option<&'a Self::Slice>;

    fn source_loc(&self) -> Self::SourceLoc;
    fn source_span(&self) -> Range<Self::SourceLoc>;

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

pub struct CharStream<'a> {
    chars: Chars<'a>,
    len: usize,
}

impl<'a> CharStream<'a> {
    #[inline]
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars(),
            len: s.len(),
        }
    }
}

impl<'a> Stream<'a> for CharStream<'a> {
    type Token = char;
    type Slice = str;

    type SourceLoc = usize;

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.chars.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.chars.next()
    }

    #[inline]
    fn peek_slice(&self, len: usize) -> Option<&'a Self::Slice> {
        self.chars
            .as_str()
            .split_at_checked(len)
            .map(|(slice, _)| slice)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<&'a Self::Slice> {
        let (slice, rest) = self.chars.as_str().split_at_checked(len)?;
        self.chars = rest.chars();
        Some(slice)
    }

    #[inline]
    fn source_loc(&self) -> Self::SourceLoc {
        self.len - self.chars.as_str().len()
    }

    #[inline]
    fn source_span(&self) -> Range<Self::SourceLoc> {
        let start = self.source_loc();
        let end = start + self.peek_token().map(|c| c.len_utf8()).unwrap_or(0);
        start..end
    }
}

pub struct SliceIter<'a, T: SourceSpanned> {
    iter: Iter<'a, T>,
    end: T::SourcePosition,
}

impl<'a, T: SourceSpanned + Clone + PartialEq> SliceIter<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T]) -> Self {
        Self {
            iter: slice.iter(),
            end: slice
                .last()
                .map(|token| token.source_span().end)
                .unwrap_or_default(),
        }
    }
}

impl<'a, T: SourceSpanned + Clone + PartialEq> Stream<'a> for SliceIter<'a, T> {
    type Token = &'a T;
    type Slice = [T];

    type SourceLoc = T::SourcePosition;

    #[inline]
    fn peek_token(&self) -> Option<Self::Token> {
        self.iter.clone().next()
    }

    #[inline]
    fn next_token(&mut self) -> Option<Self::Token> {
        self.iter.next()
    }

    #[inline]
    fn peek_slice(&self, len: usize) -> Option<&'a Self::Slice> {
        self.iter
            .as_slice()
            .split_at_checked(len)
            .map(|(slice, _)| slice)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<&'a Self::Slice> {
        let (slice, rest) = self.iter.as_slice().split_at_checked(len)?;
        self.iter = rest.iter();
        Some(slice)
    }

    #[inline]
    fn source_loc(&self) -> Self::SourceLoc {
        self.peek_token()
            .map(|token| token.source_span().start)
            .unwrap_or_else(|| self.end.clone())
    }

    #[inline]
    fn source_span(&self) -> Range<Self::SourceLoc> {
        self.peek_token()
            .map(|token| token.source_span())
            .unwrap_or_else(|| self.end.clone()..self.end.clone())
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

    type SourceLoc = S::SourceLoc;

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
    fn source_loc(&self) -> Self::SourceLoc {
        self.stream.source_loc()
    }

    #[inline]
    fn source_span(&self) -> Range<Self::SourceLoc> {
        self.stream.source_span()
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

pub trait SourceSpanned {
    type SourcePosition: Default + Clone;

    fn source_span(&self) -> Range<Self::SourcePosition>;
}
