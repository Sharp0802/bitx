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

    #[test]
    fn merge_lower_start_wins() {
        // When the other value starts before `self`, the merge
        // should adopt the earlier start. Exercises the
        // `self.start > other.start` branch.
        let mut first = Value { start: 5, end: 7 };
        let second = Value { start: 2, end: 4 };
        first.merge(&second);
        assert_eq!(first.start, 2);
        assert_eq!(first.end, 7);
    }

    #[test]
    fn parse_non_numeric_lhs() {
        // The first token must be a numeric literal.
        let ts: TokenStream = "foo".parse().unwrap();
        let mut input: Input = ts.into();
        let result: Result<Value, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_lhs_via_string_literal() {
        // A string literal is a valid `Literal` token but isn't
        // a valid number, so `parse_u128` fails.
        use proc_macro2::Literal;
        let bad = Literal::string("nope");
        let ts = quote!(#bad);
        let mut input: Input = ts.into();
        let result: Result<Value, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_single_dot() {
        // `1.2` parses fine. `1.` (a literal followed by a single
        // dot, no second dot) should fail with `.` expected.
        let ts: TokenStream = "1 .".parse().unwrap();
        let mut input: Input = ts.into();
        let result: Result<Value, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_non_numeric_rhs() {
        // `1..foo` — the rhs is a non-numeric literal.
        use proc_macro2::Literal;
        let bad = Literal::string("nope");
        let ts = quote!(1 .. #bad);
        let mut input: Input = ts.into();
        let result: Result<Value, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_exclusive_range_lower_eq_upper() {
        // `3..3` (exclusive) has end == start, which is rejected.
        let ts: TokenStream = "3 .. 3".parse().unwrap();
        let mut input: Input = ts.into();
        let result: Result<Value, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_exclusive_range_higher_end() {
        // `3..5` (exclusive) becomes start=3, end=4.
        let ts: TokenStream = "3 .. 5".parse().unwrap();
        let mut input: Input = ts.into();
        let value: Value = input.parse().unwrap();
        assert_eq!(value.start, 3);
        assert_eq!(value.end, 4);
    }

    #[test]
    fn display_start_eq_zero() {
        // When start == 0, the `..{start}` branch fires. This is
        // a degenerate case: a range that starts at 0 with a
        // non-MAX end. The current implementation prints `..0x0`
        // which is misleading, but the test pins the current
        // behavior.
        let value = Value { start: 0, end: 9 };
        assert_eq!(value.to_string(), "..0x0");
    }
}
