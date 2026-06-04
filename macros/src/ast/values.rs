use crate::ast::Value;
use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_value(start: u128, end: u128) -> Value {
        Value { start, end }
    }

    fn point_value(value: u128) -> Value {
        make_value(value, value)
    }

    fn range_value(start: u128, end: u128) -> Value {
        make_value(start, end)
    }

    #[test]
    fn is_point_and_point() {
        let one = Values::from(vec![point_value(5)]);
        assert!(one.is_point());
        assert_eq!(one.point(), Some(5));

        let two = Values::from(vec![range_value(1, 3)]);
        assert!(!two.is_point());
        assert_eq!(two.point(), None);
    }

    #[test]
    fn is_empty_for_default() {
        // '_' produces an empty Values
        let ts = quote!(_);
        let mut input: Input = ts.into();
        let parsed: Values = input.parse().unwrap();
        assert!(parsed.is_empty());
    }

    #[test]
    fn bounds_single() {
        let single = Values::from(vec![make_value(3, 5)]);
        let bounds = single.bounds().unwrap();
        assert_eq!(bounds.start, 3);
        assert_eq!(bounds.end, 5);
    }

    #[test]
    fn bounds_two_produces_gap() {
        let pair = Values::from(vec![make_value(0, 1), make_value(4, 5)]);
        let gaps = pair.bounds().unwrap_err();
        // The gap between 0..=1 and 4..=5 is 2..=3
        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0].start, 2);
        assert_eq!(gaps[0].end, 3);
    }

    #[test]
    fn no_overlap_passes_through() {
        let raw = vec![make_value(0, 1), make_value(3, 5)];
        let result = Values::no_overlap(raw).unwrap();
        assert_eq!(result.iter().count(), 2);
    }

    #[test]
    fn no_overlap_merges_adjacent() {
        let raw = vec![make_value(0, 2), make_value(3, 5)];
        let result = Values::no_overlap(raw).unwrap();
        assert_eq!(result.iter().count(), 1);
        let merged = result.iter().next().unwrap();
        assert_eq!(merged.start, 0);
        assert_eq!(merged.end, 5);
    }

    #[test]
    fn no_overlap_rejects_overlap() {
        let raw = vec![make_value(0, 4), make_value(3, 5)];
        let offending = Values::no_overlap(raw).unwrap_err();
        assert_eq!(offending.start, 3);
        assert_eq!(offending.end, 5);
    }

    #[test]
    fn from_vec_merges_adjacent() {
        let raw = vec![make_value(0, 1), make_value(2, 3)];
        let result: Values = raw.into();
        assert_eq!(result.iter().count(), 1);
    }

    #[test]
    fn display_truncates_long() {
        // Non-adjacent points so the merge logic in `From<Vec<Value>>`
        // doesn't collapse them into a single range.
        let raw: Vec<Value> = [0u128, 10, 20, 30, 40]
            .iter()
            .map(|value| point_value(*value))
            .collect();
        let vals = Values::from(raw);
        let text = vals.to_string();
        // First 3 values are shown, then ` | ...`
        assert!(
            text.contains("..."),
            "expected truncation marker, got {text}",
        );
    }

    #[test]
    fn no_overlap_empty_vec() {
        // Empty input short-circuits to Ok(empty).
        let result = Values::no_overlap(vec![]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn no_overlap_single_value() {
        // One-element input also short-circuits.
        let result = Values::no_overlap(vec![make_value(3, 5)]).unwrap();
        assert_eq!(result.iter().count(), 1);
    }

    #[test]
    fn from_empty_vec() {
        // `From<Vec<Value>>` for an empty vec hits the
        // `unreachable!()` branch — there is no first element.
        let result: Values = Vec::<Value>::new().into();
        assert!(result.is_empty());
    }
}
