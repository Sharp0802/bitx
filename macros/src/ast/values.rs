use crate::ast::Value;
use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};
use std::fmt::{Display, Formatter};

pub struct Values(Vec<Value>);

impl Parse for Values {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let mut values = Vec::new();

        if is!(input.peek(); Punct '_') {
            _ = input.pop();
            return Ok(Self(values));
        }

        values.push(input.parse()?);

        while is!(input.peek(); Punct '|') {
            _ = input.pop();
            values.push(input.parse()?);
        }

        Ok(Self(values))
    }
}

impl Values {
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn is_point(&self) -> bool {
        self.0.len() == 1 && (self.0[0].start == self.0[0].end)
    }

    #[inline]
    pub fn point(&self) -> Option<u128> {
        if self.is_point() {
            Some(self.0[0].start)
        } else {
            None
        }
    }

    pub fn bounds(&self) -> Result<Value, Vec<Value>> {
        if self.0.len() == 1 {
            Ok(Value {
                start: self.0[0].start,
                end: self.0[self.0.len() - 1].end,
            })
        } else {
            Err(self
                .0
                .windows(2)
                .map(|pair| Value {
                    start: pair[0].end + 1,
                    end: pair[1].start - 1,
                })
                .collect())
        }
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }

    pub fn no_overlap(mut raw: Vec<Value>) -> Result<Self, Value> {
        if raw.len() <= 1 {
            return Ok(Self(raw));
        }

        raw.sort_unstable();

        let mut merged = Vec::with_capacity(raw.len());

        let mut iter = raw.into_iter();
        let mut buffer = iter.next().unwrap();

        for item in iter {
            if buffer.overlap(&item) {
                return Err(item);
            } else if buffer.adjoin(&item) {
                buffer.merge(&item);
            } else {
                merged.push(std::mem::replace(&mut buffer, item));
            }
        }

        merged.push(buffer);

        Ok(Self(merged))
    }
}

impl Display for Values {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..(3.min(self.0.len())) {
            if i > 0 {
                write!(f, " | ")?;
            }

            write!(f, "{}", self.0[i])?;
        }

        if self.0.len() > 3 {
            write!(f, " | ...")?;
        }

        Ok(())
    }
}

impl From<Vec<Value>> for Values {
    fn from(mut raw: Vec<Value>) -> Self {
        if raw.len() <= 1 {
            return Self(raw);
        }

        raw.sort_unstable();

        let mut merged = Vec::with_capacity(raw.len());

        let mut iter = raw.into_iter();
        let Some(mut buffer) = iter.next() else {
            unreachable!()
        };

        for item in iter {
            if buffer.adjoin(&item) {
                buffer.merge(&item);
            } else {
                merged.push(std::mem::replace(&mut buffer, item));
            }
        }

        merged.push(buffer);

        Self(merged)
    }
}
