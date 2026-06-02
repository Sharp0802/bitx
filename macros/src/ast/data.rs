use crate::ast::{Field, Variant};
use crate::tt::*;
use crate::prelude::*;

pub enum Body {
    Enum(Block<Variant>),
    Struct(Block<Field>),
}

pub struct Data {
    pub attr: Attr,
    pub vis: Visibility,
    pub name: Ident,
    pub size: u32,
    pub body: Body,
}

impl Parse for Data {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let attr = input.parse()?;
        let vis: Visibility = input.parse()?;

        let is_struct = tok! {
            input.pop();

            Ident "struct" => true,
            Ident "enum" => false,
            _ => return Err(input.error("`struct` or `enum` expected")),
        };

        let name = input.parse()?;

        if !is!(input.pop(); Punct ':') {
            return Err(input.error("`:` expected"));
        }

        let repr: Ident = input.parse()?;
        let repr_str = repr.to_string();

        let Some(size) = repr_str.strip_prefix('u').and_then(|bits| bits.parse::<u32>().ok()) else {
            return Err(input.error("repr must be `uN` (e.g. u1, u8, etc.)"));
        };

        let body = if is_struct {
            Body::Struct(input.parse()?)
        } else {
            Body::Enum(input.parse()?)
        };

        Ok(Self {
            attr,
            vis,
            name,
            size,
            body,
        })
    }
}
