use crate::ast;
use crate::hir::Layout;
use crate::prelude::*;
use crate::tt::{Attr, Error, Type, Visibility};

#[derive(Debug)]
pub struct Field {
    pub attr: Attr,
    pub layout: Layout,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
    pub builtin: bool,
}

impl TryFrom<ast::Field> for Field {
    type Error = Error;

    fn try_from(ast: ast::Field) -> Result<Self, Error> {
        let layout: Layout = ast.layout.into();

        let builtin = ast.ty.is_none();

        let ty = if let Some(ty) = ast.ty {
            ty
        } else {
            match layout.size {
                1 => Type::boolean(),
                size @ 2..=128 => {
                    let size = size.div_ceil(8).next_power_of_two() * 8;
                    Type::literal(size)
                }

                0 => {
                    return Err(Error::new(
                        "zero size field is not allowed",
                        ast.name.span(),
                    ));
                }
                _ => {
                    return Err(Error::new(
                        "implicitly typed field cannot be larger than 128 bits",
                        ast.name.span(),
                    ));
                }
            }
        };

        Ok(Self {
            attr: ast.attr,
            layout,
            vis: ast.vis,
            name: ast.name,
            ty,
            builtin,
        })
    }
}
