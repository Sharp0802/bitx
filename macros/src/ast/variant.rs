use crate::ast::Values;
use crate::prelude::*;
use crate::tt::{Attr, Error, Input, Parse, Token};

pub struct Variant {
    pub attr: Attr,
    pub name: Ident,
    pub values: Values,
}

impl Parse for Variant {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let attr = input.parse()?;
        let values = input.parse()?;

        if !is!(input.pop(); Punct '=') || !is!(input.pop(); Punct '>') {
            return Err(input.error("`=>` expected"));
        }

        let name = input.parse()?;

        Ok(Self { attr, name, values })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ok() {
        let ts = quote!(0 => A);
        let mut input = Input::from(ts);
        let variant: Variant = input.parse().expect("variant should parse");
        assert_eq!(variant.name.to_string(), "A");
        assert!(variant.values.is_point());
    }

    #[test]
    fn parse_default() {
        let ts = quote!(_ => B);
        let mut input = Input::from(ts);
        let variant: Variant =
            input.parse().expect("default variant should parse");
        assert!(variant.values.is_empty());
    }

    #[test]
    fn parse_missing_arrow() {
        // `0 A` — no `=>` separator.
        let ts = quote!(0 A);
        let mut input = Input::from(ts);
        let result: Result<Variant, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn parse_only_one_arrow_half() {
        // `0 = A` — only the `=` of `=>`, missing the `>`.
        let ts = quote!(0 = A);
        let mut input = Input::from(ts);
        let result: Result<Variant, _> = input.parse();
        assert!(result.is_err());
    }
}
