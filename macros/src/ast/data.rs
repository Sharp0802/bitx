use crate::ast::{Field, Variant};
use crate::prelude::*;
use crate::tt::{Attr, Block, Error, Input, Parse, Token, Visibility};

#[derive(Debug)]
pub enum Body {
    Enum(Block<Variant>),
    Struct(Block<Field>),
}

#[derive(Debug)]
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

        let Some(size) = repr_str
            .strip_prefix('u')
            .and_then(|bits| bits.parse::<u32>().ok())
        else {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_struct() {
        let ts = quote! {
            pub struct Header: u16 {
                0.0;4 pub version,
            }
        };
        let mut input = Input::from(ts);
        let data: Data = input.parse().expect("struct should parse");

        assert!(matches!(data.body, Body::Struct(_)));
        assert_eq!(data.size, 16);
        assert_eq!(data.name.to_string(), "Header");
    }

    #[test]
    fn parse_enum() {
        let ts = quote! {
            pub enum Mode: u2 {
                0 => Off,
                1 => On,
            }
        };
        let mut input = Input::from(ts);
        let data: Data = input.parse().expect("enum should parse");

        assert!(matches!(data.body, Body::Enum(_)));
        assert_eq!(data.size, 2);
    }

    #[test]
    fn parse_with_attributes() {
        let ts = quote! {
            #[derive(Debug)]
            pub struct S: u8 {
                0.0;8 pub field,
            }
        };
        let mut input = Input::from(ts);
        let data: Data = input.parse().expect("struct with attrs should parse");
        let mut out = TokenStream::new();
        data.attr.to_tokens(&mut out);
        assert!(out.to_string().contains("derive"));
    }

    #[test]
    fn parse_missing_keyword() {
        // Just an ident, no `struct` or `enum`.
        let ts = quote!(pub Foo: u8 {});
        let mut input = Input::from(ts);
        let result: Result<Data, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_missing_colon() {
        // `pub struct Foo u8 { ... }` — no `:` after the name.
        let ts = quote! {
            pub struct Foo u8 {
                0.0;8 pub field,
            }
        };
        let mut input = Input::from(ts);
        let result: Result<Data, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_invalid_repr() {
        // `i8` is not a `uN` repr.
        let ts = quote! {
            pub struct S: i8 {
                0.0;8 pub field,
            }
        };
        let mut input = Input::from(ts);
        let result: Result<Data, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_repr_with_no_digits() {
        // `u` alone is not a valid repr.
        let ts = quote! {
            pub struct S: u {
                0.0;8 pub field,
            }
        };
        let mut input = Input::from(ts);
        let result: Result<Data, _> = input.parse();
        assert!(result.is_err());
    }
}
