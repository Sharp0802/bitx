use proc_macro2::Delimiter;

use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};

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
