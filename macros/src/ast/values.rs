use crate::ast::Value;
use crate::prelude::*;
use crate::tt::{Error, Input, Parse};
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct Values(Vec<Value>);

impl Parse for Values {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let mut values = Vec::new();

        if is!(input.peek(); Ident "_") {
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
    #[allow(
        clippy::missing_const_for_fn,
        reason = "const fn of `Vec::is_empty` was not stablized at MSRV 1.85"
    )]
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
        if self.is_empty() {
            Ok(Value {
                start: 0,
                end: u128::MAX,
            })
        } else if self.0.len() == 1 {
            Ok(self.0[0])
        } else {
            // NOTE: we cannot assume it's already merged.
            let merged: Self = self.0.clone().into();

            if merged.0.len() == 1 {
                Ok(merged.0[0])
            } else {
                Err(merged
                    .0
                    .windows(2)
                    .map(|pair| Value {
                        start: pair[0].end + 1,
                        end: pair[1].start - 1,
                    })
                    .collect())
            }
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
        if self.0.is_empty() {
            write!(f, "_")?;
            return Ok(());
        }

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
        let mut buffer = iter.next().unwrap_or_else(|| unreachable!());

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

#[cfg(test)]
mod tests {
    use super::*;

    tst!(Values {
        empty: "_" @|val| {
            assert!(val.is_empty());
            assert_eq!(val.to_string(), "_");
            assert_eq!(val.bounds(), Ok(Value { start: 0, end: u128::MAX }));
        },
        single: "1..=1" @|val| {
            assert_eq!(val.point(), Some(1));
            assert_eq!(val.to_string(), "0x1");
            assert_eq!(val.bounds(), Ok(Value { start: 1, end: 1 }));
            assert!(Values::no_overlap(val.0).is_ok());
        },
        multi: "1 | 2" @|val| {
            assert_eq!(val.point(), None);
            assert_eq!(val.to_string(), "0x1 | 0x2");
            assert_eq!(val.bounds(), Ok(Value { start: 1, end: 2 }));
            assert!(Values::no_overlap(val.0).is_ok());
        },
        gaps: "1 | 3..4 | 4..5" @|val| {
            assert_eq!(val.bounds(), Err(vec![ Value { start: 2, end: 2 } ]));
            assert!(Values::no_overlap(val.0).is_ok());
        },
        overlap: "1 | 1" @|val| {
            assert!(Values::no_overlap(val.0).is_err());
        },
        long: "1 | 2 | 3 | 4" Display("0x1 | 0x2 | 0x3 | ..."),
        iter: "1 | 2" @|val| {
            let mut iter = val.iter();
            assert_eq!(iter.next(), Some(&Value { start: 1, end: 1 }));
            assert_eq!(iter.next(), Some(&Value { start: 2, end: 2 }));
            assert_eq!(iter.next(), None);
        },
    });

    #[test]
    fn test_from_trivials() {
        let unit = Value { start: 1, end: 1 };

        assert_eq!(Values::from(vec![]).0, vec![]);
        assert_eq!(Values::from(vec![unit]).0, vec![unit]);
    }
}
