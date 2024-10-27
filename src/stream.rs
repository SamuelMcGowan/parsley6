use std::ops::{Deref, Range};
use std::slice::Iter;
use std::str::Chars;

pub trait Stream {
    type Token: Clone + PartialEq;

    type Slice: PartialEq + ?Sized;
    type SliceRef: Deref<Target = Self::Slice> + Copy;

    type SourceLoc: Default + Clone + Ord;

    fn peek_token(&self) -> Option<Self::Token>;
    fn next_token(&mut self) -> Option<Self::Token>;

    fn peek_slice(&self, len: usize) -> Option<Self::SliceRef>;
    fn next_slice(&mut self, len: usize) -> Option<Self::SliceRef>;

    fn stream_position(&self) -> usize;

    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef>;

    fn peek_token_span(&self) -> Range<Self::SourceLoc>;
    fn prev_token_span(&self) -> Range<Self::SourceLoc>;

    #[inline]
    fn at_end(&self) -> bool {
        self.peek_token().is_none()
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> Self::SliceRef {
        self.try_slice(start, end).expect("slice out of bounds")
    }
}

pub struct CharStream<'a> {
    all: &'a str,
    chars: Chars<'a>,
}

impl<'a> CharStream<'a> {
    #[inline]
    pub fn new(s: &'a str) -> Self {
        Self {
            all: s,
            chars: s.chars(),
        }
    }
}

impl<'a> Stream for CharStream<'a> {
    type Token = char;

    type Slice = str;
    type SliceRef = &'a str;

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
    fn peek_slice(&self, len: usize) -> Option<Self::SliceRef> {
        self.chars
            .as_str()
            .split_at_checked(len)
            .map(|(slice, _)| slice)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<Self::SliceRef> {
        let (slice, rest) = self.chars.as_str().split_at_checked(len)?;
        self.chars = rest.chars();
        Some(slice)
    }

    #[inline]
    fn stream_position(&self) -> usize {
        self.all.len() - self.chars.as_str().len()
    }

    #[inline]
    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef> {
        self.all.get(start..end)
    }

    #[inline]
    fn peek_token_span(&self) -> Range<Self::SourceLoc> {
        let pos = self.stream_position();
        let ch_len = self.peek_token().map(char::len_utf8).unwrap_or_default();
        pos..(pos + ch_len)
    }

    #[inline]
    fn prev_token_span(&self) -> Range<Self::SourceLoc> {
        let pos = self.stream_position();
        let ch_len = self.all[..pos]
            .chars()
            .next_back()
            .map(char::len_utf8)
            .unwrap_or_default();
        (pos - ch_len)..pos
    }
}

pub struct SliceIter<'a, T: SourceSpanned> {
    all: &'a [T],
    iter: Iter<'a, T>,
    end: T::SourcePosition,
}

impl<'a, T: SourceSpanned + Clone + PartialEq> SliceIter<'a, T> {
    #[inline]
    pub fn new(slice: &'a [T], end: T::SourcePosition) -> Self {
        Self {
            all: slice,
            iter: slice.iter(),
            end,
        }
    }
}

impl<'a, T: SourceSpanned + Clone + PartialEq> Stream for SliceIter<'a, T> {
    type Token = &'a T;

    type Slice = [T];
    type SliceRef = &'a [T];

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
    fn peek_slice(&self, len: usize) -> Option<Self::SliceRef> {
        self.iter
            .as_slice()
            .split_at_checked(len)
            .map(|(slice, _)| slice)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<Self::SliceRef> {
        let (slice, rest) = self.iter.as_slice().split_at_checked(len)?;
        self.iter = rest.iter();
        Some(slice)
    }

    #[inline]
    fn stream_position(&self) -> usize {
        self.all.len() - self.iter.as_slice().len()
    }

    #[inline]
    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef> {
        self.all.get(start..end)
    }

    #[inline]
    fn peek_token_span(&self) -> Range<Self::SourceLoc> {
        self.peek_token()
            .map(|t| t.source_span())
            .unwrap_or_else(|| self.end.clone()..self.end.clone())
    }

    #[inline]
    fn prev_token_span(&self) -> Range<Self::SourceLoc> {
        self.all[..self.stream_position()]
            .last()
            .map(|t| t.source_span())
            .unwrap_or_default()
    }
}

pub struct StreamWithState<S: Stream, State> {
    pub stream: S,
    pub state: State,
}

impl<S: Stream, State> StreamWithState<S, State> {
    #[inline]
    pub fn new(stream: S, state: State) -> Self {
        Self { stream, state }
    }
}

impl<S: Stream, State> Stream for StreamWithState<S, State> {
    type Token = S::Token;

    type Slice = S::Slice;
    type SliceRef = S::SliceRef;

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
    fn peek_slice(&self, len: usize) -> Option<Self::SliceRef> {
        self.stream.peek_slice(len)
    }

    #[inline]
    fn next_slice(&mut self, len: usize) -> Option<Self::SliceRef> {
        self.stream.next_slice(len)
    }

    #[inline]
    fn stream_position(&self) -> usize {
        self.stream.stream_position()
    }

    #[inline]
    fn try_slice(&self, start: usize, end: usize) -> Option<Self::SliceRef> {
        self.stream.try_slice(start, end)
    }

    #[inline]
    fn peek_token_span(&self) -> Range<Self::SourceLoc> {
        self.stream.peek_token_span()
    }

    #[inline]
    fn prev_token_span(&self) -> Range<Self::SourceLoc> {
        self.stream.prev_token_span()
    }

    #[inline]
    fn at_end(&self) -> bool {
        self.stream.at_end()
    }
}

pub trait BorrowState: crate::sealed::Sealed {
    type State;

    // TODO: do we need both of these?
    fn borrow_state(&self) -> &Self::State;
    fn borrow_state_mut(&mut self) -> &mut Self::State;
}

impl<S: Stream, State> crate::sealed::Sealed for StreamWithState<S, State> {}

impl<S: Stream, State> BorrowState for StreamWithState<S, State> {
    type State = State;

    #[inline]
    fn borrow_state(&self) -> &Self::State {
        &self.state
    }

    #[inline]
    fn borrow_state_mut(&mut self) -> &mut Self::State {
        &mut self.state
    }
}

pub trait SourceSpanned {
    type SourcePosition: Default + Clone + Ord;

    fn source_span(&self) -> Range<Self::SourcePosition>;
}

pub(crate) fn merge_spans_right<T: Ord>(start: Range<T>, end: Range<T>) -> Range<T> {
    start.start..end.end.max(start.end)
}
