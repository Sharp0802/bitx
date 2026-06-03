use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};

pub struct Block<T>(Vec<T>);

impl<T> Block<T> {
    #[inline]
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
        if !is!(input.pop(); Punct '{') {
            return Err(input.error("`{` expected"));
        }

        if is!(input.peek(); Punct '}') {
            return Ok(Self(Vec::new()));
        }

        let mut ret = Vec::new();
        loop {
            let item: T = input.parse()?;
            ret.push(item);

            tok! {
                input.pop();

                Punct '}' => break,
                Punct ',' => { /* continue */ },
                _ => return Err(input.error("`}` or `,` expected")),
            }
        }

        Ok(Self(ret))
    }
}
