use crate::prelude::*;
use crate::tt::{Error, Token};
use proc_macro2::token_stream::IntoIter;

pub trait Parse: Sized {
    fn parse(input: &mut Input) -> Result<Self, Error>;
}

pub struct Input {
    iter: IntoIter,
    buffer: Option<Token>,
    span: Span,
    error: bool,
}

impl Input {
    #[inline]
    fn next_raw(&mut self) -> Token {
        let token: Token = self.iter.next().into();
        self.span = token.span();
        token
    }

    #[inline]
    pub fn peek(&mut self) -> &Token {
        assert!(!self.error, "input has been poisoned");

        if self.buffer.is_none() {
            self.buffer = Some(self.next_raw());
        }

        self.buffer.as_ref().unwrap()
    }

    #[inline]
    pub fn pop(&mut self) -> Token {
        assert!(!self.error, "input has been poisoned");

        if let Some(item) = self.buffer.take() {
            return item;
        }

        self.next_raw()
    }

    #[inline]
    pub fn parse<T: Parse>(&mut self) -> Result<T, Error> {
        T::parse(self)
    }

    #[inline]
    pub fn error(&mut self, message: &'static str) -> Error {
        // NOTE: error is an error.
        //       do NOT attempt to recover state.
        self.error = true;
        Error::new(message, self.span)
    }
}

impl From<TokenStream> for Input {
    fn from(ts: TokenStream) -> Self {
        Self {
            iter: ts.into_iter(),
            buffer: None,
            span: Span::call_site(),
            error: false,
        }
    }
}
