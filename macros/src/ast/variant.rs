use crate::ast::Values;
use crate::prelude::*;

pub struct Variant {
    pub name: Ident,
    pub values: Values,
}

pub struct Variants(Punctuated<Variant, Token![,]>);

impl Parse for Variant {
    fn parse(input: ParseStream) -> Result<Self> {
        let values = input.parse()?;
        let name = input.parse()?;

        Ok(Self { name, values })
    }
}

impl Parse for Variants {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::braced!(content in input);
        let vars = content.parse_terminated(Variant::parse, Token![,])?;
        Ok(Self(vars))
    }
}

impl Variants {
    pub fn into_iter(self) -> impl Iterator<Item = Variant> {
        self.0.into_iter()
    }
}

