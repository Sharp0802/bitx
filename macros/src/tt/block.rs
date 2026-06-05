use proc_macro2::Delimiter;

use crate::prelude::*;
use crate::tt::{Error, Input, Parse};

#[derive(Debug)]
pub struct Block<T>(Vec<T>);

impl<T> Block<T> {
    #[inline]
    #[allow(
        clippy::missing_const_for_fn,
        reason = "const fn of `Vec::is_empty` was not stablized at MSRV 1.85"
    )]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }

    #[inline]
    pub fn into_iter(self) -> std::vec::IntoIter<T> {
        self.0.into_iter()
    }
}

impl<T: Parse> Parse for Block<T> {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let group: Group = input.parse()?;
        if group.delimiter() != Delimiter::Brace {
            let span = group.delim_span().join();
            return Err(Error::new("`{` and `}` are expected", span));
        }

        let mut input: Input = group.stream().into();

        let mut ret = Vec::new();
        loop {
            if is!(input.peek(); End) {
                break;
            }

            let item: T = input.parse()?;
            ret.push(item);

            tok! {
                input.pop();

                End => break,
                Punct ',' => { /* continue */ },
                _ => return Err(input.error("`}` or `,` expected")),
            }
        }

        Ok(Self(ret))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let ts = quote!({});
        let mut input = Input::from(ts);
        let block: Block<Ident> = input.parse().unwrap();
        assert!(block.is_empty());
    }

    #[test]
    fn parse_items() {
        let ts = quote!({ a, b, c });
        let mut input = Input::from(ts);
        let block: Block<Ident> = input.parse().unwrap();
        let names: Vec<String> =
            block.iter().map(ToString::to_string).collect();
        assert_eq!(names, vec!["a", "b", "c"]);
    }

    #[test]
    fn parse_trailing_comma() {
        let ts = quote!({ a, b, });
        let mut input = Input::from(ts);
        let block: Block<Ident> = input.parse().unwrap();
        let names: Vec<String> =
            block.iter().map(ToString::to_string).collect();
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn deny_wrong_delimiter() {
        let ts = quote!((a));
        let mut input = Input::from(ts);
        let result: Result<Block<Ident>, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn deny_missing_separator() {
        let ts = quote!({ a b });
        let mut input = Input::from(ts);
        let result: Result<Block<Ident>, _> = input.parse();
        assert!(result.is_err());
    }
}
