use crate::prelude::*;
use crate::tt::{Attr, Error, Input, Layout, Parse, Type, Visibility};

#[derive(Debug)]
pub struct Field {
    pub attr: Attr,
    pub layout: Layout,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Option<Type>,
}

impl Parse for Field {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let attr = input.parse()?;
        let layout = input.parse()?;
        let vis = input.parse()?;
        let name = input.parse()?;

        let ty = if is!(input.peek(); Punct ':') {
            _ = input.pop();
            let ty: Type = input.parse()?;
            Some(ty)
        } else {
            None
        };

        Ok(Self {
            attr,
            layout,
            vis,
            name,
            ty,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    tst!(Field {
        with_ty: "0.0;0 foo : T" Ok,
        without_ty: "0.0;0 foo" Ok,
    });
}
