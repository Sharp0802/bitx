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

    fn try_from(ast: Data) -> Result<Self, Error> {
        let Body::Struct(fields) = ast.body else {
            panic!("struct expected");
        };

        let fields: Vec<Field> = fields.into_iter().map(Into::into).collect();

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
