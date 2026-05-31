use crate::ast;
use crate::hir::{Mask, Value, Values};
use crate::prelude::*;

pub struct Variant {
    pub name: Ident,
    pub values: Values,
}

pub struct Enum {
    pub vis: Visibility,
    pub name: Ident,
    pub size: Offset,
    pub mask: Mask,
    pub variants: Vec<Variant>,
    pub is_sealed: bool,
}

impl TryFrom<ast::Variant> for Variant {
    type Error = Error;

    fn try_from(ast: ast::Variant) -> Result<Self> {
        Ok(Self {
            name: ast.name,
            values: ast.values.try_into()?,
        })
    }
}

impl TryFrom<ast::Data> for Enum {
    type Error = Error;

    fn try_from(ast: ast::Data) -> Result<Self> {
        let ast::Body::Variants(variants) = ast.body else {
            panic!("enum expected");
        };

        let max = match ast.size.bits() {
            128 => u128::MAX,
            bits if bits < 128 => (1u128 << bits) - 1,
            _ => {
                return Err(Error::new(
                    ast.name.span(),
                    "enum cannot be larger than 128 bits",
                ));
            }
        };

        let variants: Vec<Variant> = variants
            .into_iter()
            .map(|variant| variant.try_into())
            .collect::<Result<Vec<_>>>()?;
        if variants.is_empty() {
            return Err(Error::new(
                ast.name.span(),
                "zero-variant enum is not allowed",
            ));
        }

        let mut has_default = false;
        for variant in &variants {
            has_default |= variant.values.is_empty();

            for value in variant.values.iter() {
                if value.end <= max {
                    continue;
                }

                return Err(Error::new(
                    variant.name.span(),
                    "value of variant exceeds maximum value of enum (`{max}`)",
                ));
            }
        }

        let values: Values = match Values::no_overlap(
            variants
                .iter()
                .flat_map(|var| var.values.iter())
                .cloned()
                .collect::<Vec<_>>(),
        ) {
            Ok(values) => values,
            Err(at) => {
                let mut who = &variants[0];

                let mut i = 0;
                for variant in &variants {
                    i += variant.values.len();
                    if at < i {
                        who = variant;
                    }
                }

                return Err(Error::new(
                    who.name.span(),
                    "variant is overlapped with previous variants",
                ));
            }
        };

        let is_sealed = if !has_default {
            match values.bounds() {
                Ok(bounds) if 0 < bounds.start => {
                    return Err(Error::new(
                        ast.name.span(),
                        &format!("enum has uncovered case: ..{}", bounds.start),
                    ));
                }
                Ok(bounds) if bounds.end < max => {
                    return Err(Error::new(
                        ast.name.span(),
                        &format!(
                            "enum has uncovered case: {}..{max}",
                            bounds.end + 1,
                        ),
                    ));
                }
                Err(gaps) => {
                    return Err(Error::new(
                        ast.name.span(),
                        &format!(
                            "enum has uncovered case: {}",
                            Into::<Values>::into(gaps),
                        ),
                    ));
                }

                Ok(bounds) => {}
            };

            true
        } else {
            false
        };

        let mask = Mask::for_size(ast.size).unwrap();

        Ok(Self {
            vis: ast.vis,
            name: ast.name,
            size: ast.size,
            mask,
            variants,
            is_sealed,
        })
    }
}
