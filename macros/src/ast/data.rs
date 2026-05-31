use crate::ast::{Fields, Variants};
use crate::prelude::*;

pub enum Body {
    Variants(Variants),
    Fields(Fields),
}

pub struct Data {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub size: Offset,
    pub body: Body,
}

impl Parse for Data {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let vis: Visibility = input.parse()?;

        let is_struct = if input.peek(Token![struct]) {
            _ = input.parse::<Token![struct]>()?;
            true
        } else {
            _ = input.parse::<Token![enum]>()?;
            false
        };

        let name: Ident = input.parse()?;
        _ = input.parse::<Token![:]>()?;
        let size: Offset = input.parse()?;

        let body = if is_struct {
            Body::Fields(input.parse()?)
        } else {
            Body::Variants(input.parse()?)
        };

        Ok(Self {
            attrs,
            vis,
            name,
            size,
            body,
        })
    }
}
