use crate::ast::{self, Body};
use crate::hir::{Enum, Struct};
use crate::tt::Error;

pub enum Data {
    Enum(Enum),
    Struct(Struct),
}

impl TryFrom<ast::Data> for Data {
    type Error = Error;

    fn try_from(ast: ast::Data) -> Result<Self, Error> {
        if matches!(&ast.body, Body::Enum(_)) {
            Ok(Self::Enum(ast.try_into()?))
        } else {
            Ok(Self::Struct(ast.try_into()?))
        }
    }
}
