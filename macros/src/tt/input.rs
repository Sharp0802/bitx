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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn peek_does_not_advance() {
        let ts = quote!(a b c);
        let mut input = Input::from(ts);
        let _ = input.peek();
        let _ = input.peek();
        let ident: Ident = input.parse().unwrap();
        assert_eq!(ident.to_string(), "a");
    }

    #[test]
    fn pop_advances() {
        let ts = quote!(a b);
        let mut input = Input::from(ts);
        let first: Ident = input.parse().unwrap();
        assert_eq!(first.to_string(), "a");
        let second: Ident = input.parse().unwrap();
        assert_eq!(second.to_string(), "b");
    }

    #[test]
    fn end_after_consumption() {
        let ts = quote!(a);
        let mut input = Input::from(ts);
        let _first: Ident = input.parse().unwrap();
        let end = input.pop();
        assert!(matches!(end, Token::End));
    }

    #[test]
    fn end_for_empty() {
        let ts = quote!();
        let mut input = Input::from(ts);
        assert!(matches!(input.pop(), Token::End));
    }

    #[test]
    fn error_poisons_input() {
        let ts = quote!(a);
        let mut input = Input::from(ts);
        let _ = input.error("boom");
        // Calling pop after error panics.
        let result =
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut i = input;
                let _ = i.pop();
            }));
        assert!(result.is_err());
    }

    #[test]
    fn parse_chains() {
        // Two parses on the same input should advance through it.
        let ts = quote!(a b);
        let mut input = Input::from(ts);
        let first: Ident = input.parse().unwrap();
        assert_eq!(first.to_string(), "a");
        let second: Ident = input.parse().unwrap();
        assert_eq!(second.to_string(), "b");
    }
}
