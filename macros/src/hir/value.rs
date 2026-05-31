use crate::prelude::*;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value {
    pub start: u128,
    pub end: u128,
}

#[derive(Clone)]
pub struct Values(Vec<Value>);

impl Value {
    #[must_use]
    #[inline]
    const fn overlap(&self, other: &Self) -> bool {
        other.start <= self.end.saturating_add(1)
            && self.start <= other.end.saturating_add(1)
    }

    #[must_use]
    #[inline]
    const fn overlap_strict(&self, other: &Self) -> bool {
        other.start <= self.end && self.start <= other.end
    }

    #[inline]
    const fn merge(&mut self, other: &Self) {
        if self.start > other.start {
            self.start = other.start;
        }
        if self.end < other.end {
            self.end = other.end;
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "{:#X}", self.start)
        } else if self.end == u128::MAX {
            write!(f, "{:#X}..", self.start)
        } else if self.start == 0 {
            write!(f, "..{:#X}", self.start)
        } else {
            let start = self.start;
            let end = self.end + 1;
            let width = start.ilog2().max(end.ilog2()).div_ceil(4) as usize;

            write!(f, "{start:#0width$X}..{end:#0width$X}")
        }
    }
}

impl TryFrom<ast::Value> for Value {
    type Error = Error;

    fn try_from(value: ast::Value) -> Result<Self> {
        match value {
            ast::Value::Point(lit) => {
                let val = lit.base10_parse()?;

                Ok(Self {
                    start: val,
                    end: val,
                })
            }
            ast::Value::Range(lhs_lit, rhs_lit) => {
                let start: u128 = lhs_lit.base10_parse()?;
                let end: u128 = rhs_lit.base10_parse()?;

                if start < end {
                    Ok(Self {
                        start,
                        end: end - 1,
                    })
                } else {
                    Err(Error::new(
                        rhs_lit.span(),
                        "end of range must be greater than start",
                    ))
                }
            }
            ast::Value::RangeEq(lhs_lit, rhs_lit) => {
                let start: u128 = lhs_lit.base10_parse()?;
                let end: u128 = rhs_lit.base10_parse()?;

                if start <= end {
                    Ok(Self { start, end })
                } else {
                    Err(Error::new(
                        rhs_lit.span(),
                        "end of inclusive range must be equal to \
                         or greater than start",
                    ))
                }
            }
        }
    }
}

impl Values {
    #[must_use]
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[must_use]
    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[must_use]
    pub fn bounds(&self) -> StdResult<Value, Vec<Value>> {
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
                    start: pair[0].end,
                    end: pair[1].start,
                })
                .collect())
        }
    }

    #[must_use]
    pub fn iter(&self) -> impl Iterator<Item = &Value> {
        self.0.iter()
    }

    #[must_use]
    pub fn no_overlap(mut raw: Vec<Value>) -> StdResult<Self, usize> {
        if raw.len() <= 1 {
            return Ok(Self(raw));
        }

        raw.sort_unstable();

        let mut merged = Vec::with_capacity(raw.len());

        let mut iter = raw.into_iter();
        let mut buffer = iter.next().unwrap();

        let mut i = 0;
        for item in iter {
            if buffer.overlap_strict(&item) {
                return Err(i);
            } else if buffer.overlap(&item) {
                buffer.merge(&item);
            } else {
                merged.push(std::mem::replace(&mut buffer, item));
            }

            i += 1;
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

            write!(f, "{}", &self.0[i])?;
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
        let mut buffer = iter.next().unwrap();

        for item in iter {
            if buffer.overlap(&item) {
                buffer.merge(&item);
            } else {
                merged.push(std::mem::replace(&mut buffer, item));
            }
        }

        merged.push(buffer);

        Self(merged)
    }
}

impl TryFrom<ast::Values> for Values {
    type Error = Error;

    fn try_from(raw: ast::Values) -> Result<Self> {
        let raw: Vec<Value> = raw
            .take()
            .into_iter()
            .map(Value::try_from)
            .collect::<Result<Vec<_>>>()?;

        Ok(raw.into())
    }
}
