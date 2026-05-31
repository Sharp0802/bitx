use crate::prelude::*;

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub offset: Offset,
    pub ty: Type,
}

pub struct Fields(Punctuated<Field, Token![,]>);

impl Parse for Field {
    fn parse(input: ParseStream) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        let offset: Offset = input.parse()?;
        let vis: Visibility = input.parse()?;
        let name: Ident = input.parse()?;
        let ty: Type = input.parse()?;

        Ok(Self {
            attrs,
            vis,
            name,
            offset,
            ty,
        })
    }
}

impl Parse for Fields {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        syn::braced!(content in input);
        let fields = content.parse_terminated(Field::parse, Token![,])?;
        Ok(Self(fields))
    }
}

impl Fields {
    pub fn into_iter(self) -> impl Iterator<Item = Field> {
        self.0.into_iter()
    }
}
