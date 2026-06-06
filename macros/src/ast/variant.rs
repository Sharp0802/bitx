use crate::ast::Values;
use crate::prelude::*;
use crate::tt::{Attr, Error, Input, Parse};

#[derive(Debug)]
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

    tst!(Variant {
        no_arrow: "0 Foo" Err("`=>`"),
    });
}
