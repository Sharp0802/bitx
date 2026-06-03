use crate::ast;
use crate::hir::Layout;
use crate::prelude::*;
use crate::tt::{Attr, Type, Visibility};

pub struct Field {
    pub attr: Attr,
    pub layout: Layout,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
    pub builtin: bool,
}

impl From<ast::Field> for Field {
    fn from(ast: ast::Field) -> Self {
        let layout: Layout = ast.layout.into();

        let builtin = ast.ty.is_none();
        let ty = ast.ty.unwrap_or_else(|| {
            if layout.size == 1 {
                Type::boolean()
            } else {
                let size = layout.size.div_ceil(8).next_power_of_two() * 8;
                Type::literal(size)
            }
        });

        Self {
            attr: ast.attr,
            layout,
            vis: ast.vis,
            name: ast.name,
            ty,
            builtin,
        }
    }
}
