use crate::prelude::*;
use crate::hir::Mask;

pub enum Kind {
    Literal(Offset),
    Custom(Type),
}

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub offset: Offset,
    pub mask: Option<Mask>,
    pub kind: Kind,
}

pub struct Struct {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub name: Ident,
    pub size: Offset,
    pub mask: Option<Mask>,
    pub fields: Vec<Field>,
}

impl TryFrom<ast::Field> for Field {
    type Error = Error;

    fn try_from(value: ast::Field) -> Result<Self> {
        let (kind, mask) = if let Some(size) = lit::size_of(&value.ty) {
            (Kind::Literal(size), Mask::for_size(size))
        } else {
            (Kind::Custom(value.ty), None)
        };

        Ok(Self {
            attrs: value.attrs,
            vis: value.vis,
            name: value.name,
            offset: value.offset,
            mask,
            kind,
        })
    }
}

impl TryFrom<ast::Data> for Struct {
    type Error = Error;

    fn try_from(ast: ast::Data) -> Result<Self> {
        let ast::Body::Fields(fields) = ast.body else {
            panic!("struct expected");
        };

        let fields: Vec<Field> = fields
            .into_iter()
            .map(|field| field.try_into())
            .collect::<Result<Vec<_>>>()?;

        for field in &fields {
            let Kind::Literal(size) = field.kind else {
                continue;
            };

            if field.offset + size > ast.size {
                return Err(Error::new(
                    field.name.span(),
                    "field exceeds struct bounds",
                ));
            }
        }

        let mask = Mask::for_size(ast.size);

        Ok(Self {
            attrs: ast.attrs,
            vis: ast.vis,
            name: ast.name,
            size: ast.size,
            mask,
            fields,
        })
    }
}

