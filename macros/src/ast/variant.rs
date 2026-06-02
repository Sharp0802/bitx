use crate::ast::Values;
use crate::tt::*;
use crate::prelude::*;

pub struct Variant {
    pub name: Ident,
    pub values: Values,
}

impl Parse for Variant {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let values = input.parse()?;
        let name = input.parse()?;

        Ok(Self { name, values })
    }
}

