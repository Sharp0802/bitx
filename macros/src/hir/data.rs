use crate::hir::{Enum, Struct};
use crate::prelude::*;

pub enum Data {
    Enum(Enum),
    Struct(Struct),
}

impl TryFrom<ast::Data> for Data {
    type Error = Error;

    fn try_from(ast: ast::Data) -> Result<Self> {
        let is_enum = matches!(&ast.body, ast::Body::Variants(_));

        if is_enum {
            Ok(Self::Enum(ast.try_into()?))
        } else {
            Ok(Self::Struct(ast.try_into()?))
        }
    }
}
