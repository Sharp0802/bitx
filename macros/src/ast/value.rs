use crate::prelude::*;

pub enum Value {
    Point(LitInt),
    Range(LitInt, LitInt),
    RangeEq(LitInt, LitInt),
}

pub struct Values(Vec<Value>);

impl Parse for Value {
    fn parse(input: ParseStream) -> Result<Self> {
        let lhs = input.parse()?;

        if input.peek(Token![..=]) {
            _ = input.parse::<Token![..=]>()?;

            let rhs = input.parse()?;
            Ok(Self::RangeEq(lhs, rhs))
        } else if input.peek(Token![..]) {
            _ = input.parse::<Token![..]>()?;

            let rhs = input.parse()?;
            Ok(Self::Range(lhs, rhs))
        } else {
            Ok(Self::Point(lhs))
        }
    }
}

impl Parse for Values {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut values = Vec::new();

        if input.peek(Token![_]) {
            _ = input.parse::<Token![_]>()?;
            return Ok(Self(values));
        }

        values.push(input.parse()?);

        while input.peek(Token![|]) {
            _ = input.parse::<Token![|]>()?;

            values.push(input.parse()?);
        }

        Ok(Self(values))
    }
}

impl Values {
    pub fn take(self) -> Vec<Value> {
        self.0
    }
}

