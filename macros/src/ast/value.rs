use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token, is, parse_u128, tok};
use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Value {
    pub start: u128,
    pub end: u128,
}

impl Parse for Value {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let lhs = {
            let lit: Literal = input.parse()?;
            let Ok(lhs) = parse_u128(&lit.to_string()) else {
                return Err(input.error("start of range must be valid integer"));
            };

            lhs
        };

        tok! {
            input.peek();

            Punct '.' => {
                _ = input.pop();

                if !is!(input.pop(); Punct '.') {
                    return Err(input.error("`.` expected"));
                }

                let incl = if is!(input.peek(); Punct '=') {
                    _ = input.pop();
                    true
                } else {
                    false
                };

                let lit: Literal = input.parse()?;

                let Ok(rhs) = parse_u128(&lit.to_string()) else {
                    return Err(input.error(
                        "end of range must be valid integer"
                    ));
                };

                let rhs = if incl {
                    rhs
                } else if rhs > lhs {
                    rhs - 1
                } else {
                    return Err(input.error(
                        "end of exclusive range must be \
                         greater than start of range"
                    ));
                };

                Ok(Self {
                    start: lhs,
                    end: rhs,
                })
            },
            _ => Ok(Self {
                start: lhs,
                end: lhs,
            }),
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

impl Value {
    #[inline]
    pub const fn adjoin(&self, other: &Self) -> bool {
        other.start <= self.end.saturating_add(1)
            && self.start <= other.end.saturating_add(1)
    }

    #[inline]
    pub const fn overlap(&self, other: &Self) -> bool {
        other.start <= self.end && self.start <= other.end
    }

    #[inline]
    pub const fn merge(&mut self, other: &Self) {
        if self.start > other.start {
            self.start = other.start;
        }
        if self.end < other.end {
            self.end = other.end;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_display() {
        let value = Value { start: 5, end: 5 };
        assert_eq!(value.to_string(), "0x5");
    }

    #[test]
    fn unbounded_top_display() {
        let value = Value {
            start: 5,
            end: u128::MAX,
        };
        assert_eq!(value.to_string(), "0x5..");
    }

    #[test]
    fn general_range_display() {
        let value = Value {
            start: 0x10,
            end: 0x1F,
        };
        // end+1 = 0x20, so we render 0x10..0x20
        assert_eq!(value.to_string(), "0x10..0x20");
    }

    #[test]
    fn overlap() {
        let first = Value { start: 0, end: 5 };
        let second = Value { start: 3, end: 8 };
        assert!(first.overlap(&second));
        assert!(second.overlap(&first));
    }

    #[test]
    fn no_overlap() {
        let first = Value { start: 0, end: 2 };
        let second = Value { start: 4, end: 6 };
        assert!(!first.overlap(&second));
        assert!(!second.overlap(&first));
    }

    #[test]
    fn adjacent() {
        let first = Value { start: 0, end: 2 };
        let second = Value { start: 3, end: 6 };
        assert!(first.adjoin(&second));
        assert!(second.adjoin(&first));
        assert!(!first.overlap(&second));
    }

    #[test]
    fn merge_extends() {
        let mut first = Value { start: 0, end: 2 };
        let second = Value { start: 5, end: 7 };
        first.merge(&second);
        assert_eq!(first.start, 0);
        assert_eq!(first.end, 7);
    }
}
