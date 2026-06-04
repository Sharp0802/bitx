use crate::ast::{Body, Data};
use crate::hir::{Field, Mask};
use crate::prelude::*;
use crate::tt::{Attr, Error, Visibility};

pub struct Struct {
    pub attr: Attr,
    pub vis: Visibility,
    pub name: Ident,
    pub size: u32,
    pub mask: Option<Mask>,
    pub fields: Vec<Field>,
}

impl TryFrom<Data> for Struct {
    type Error = Error;

    #[inline(always)]
    #[expect(
        clippy::inline_always,
        reason = "1. fn is used only at `hir::Data::try_from`.\n\
                  2. prologue checks can be omitted by context.\n\
                  3. inlining may not be done without `always`."
    )]
    fn try_from(ast: Data) -> Result<Self, Error> {
        // NOTE: this check should be omitted by inlining
        let Body::Struct(fields) = ast.body else {
            return Err(Error::new(
                "internal: cannot raising enum AST into struct HIR",
                ast.name.span(),
            ));
        };

        let fields: Vec<Field> = fields
            .into_iter()
            .map(TryInto::try_into)
            .collect::<Result<_, _>>()?;

        for field in &fields {
            if field.layout.offset + field.layout.size > ast.size {
                return Err(Error::new(
                    "field exceeds struct bounds",
                    field.name.span(),
                ));
            }
        }

        let mask = Mask::new(ast.size);

        Ok(Self {
            attr: ast.attr,
            vis: ast.vis,
            name: ast.name,
            size: ast.size,
            mask,
            fields,
        })
    }
}
