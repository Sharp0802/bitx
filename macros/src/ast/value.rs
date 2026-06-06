use crate::prelude::*;
use crate::tt::{Error, Input, Parse, parse_u128};
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
            write!(f, "..={:#X}", self.end)
        } else {
            let start = self.start;
            let end = self.end;
            let width = start.ilog2().max(end.ilog2()).div_ceil(4) as usize;

            write!(f, "{start:#0width$X}..={end:#0width$X}")
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

    tst!(Value {
        point: "0" Display("0x0"),
        left_only: "1..=0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF" Display("0x1.."),
        right_only: "0..=1" Display("..=0x1"),
        range_short: "1..0x2" Display("0x1"),
        range_long: "1..0o10" Display("0x1..=0x7"),

        no_literal_start: "foo.." Err("a literal expected"),
        no_literal_end: "1..foo" Err("a literal expected"),
        invalid_start: "1.1..2" Err("start of"),
        invalid_end: "1..1.1" Err("end of"),

        single_dot: "1 . 1" Err("`.`"),

        invalid_range: "2..1" Err("greater"),
    });

    const MIN: Value = Value { start: 0, end: 0 };

    const MID: Value = Value {
        start: 1,
        end: u128::MAX - 1,
    };

    const MAX: Value = Value {
        start: u128::MAX,
        end: u128::MAX,
    };

    const FULL: Value = Value {
        start: 0,
        end: u128::MAX,
    };

    #[test]
    fn test_adjoin() {
        assert!(MIN.adjoin(&MID));
        assert!(MID.adjoin(&MAX));
        assert!(!MIN.adjoin(&MAX));
    }

    #[test]
    fn test_overlap() {
        assert!(!MIN.overlap(&MID));
        assert!(!MID.overlap(&MAX));
        assert!(!MIN.overlap(&MAX));

        assert!(FULL.overlap(&MIN));
        assert!(FULL.overlap(&MID));
        assert!(FULL.overlap(&MAX));
    }

    #[test]
    fn test_merge() {
        let mut buf = MIN;
        buf.merge(&MAX);
        assert_eq!(buf, FULL);

        let mut buf = MID;
        buf.merge(&FULL);
        assert_eq!(buf, FULL);
    }
}
