use crate::ast;
use crate::hir::{Mask, Values};
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

fn check_range<'a>(
    variants: impl Iterator<Item = &'a Variant>,
    max: u128,
) -> Result<()> {
    for variant in variants {
        for value in variant.values.iter() {
            if value.end <= max {
                continue;
            }

            return Err(Error::new(
                variant.name.span(),
                format!(
                    "value of variant exceeds \
                     maximum value of enum (`{max}`)"
                ),
            ));
        }
    }

    Ok(())
}

fn merge_values(variants: &[Variant]) -> Result<Values> {
    Values::no_overlap(
        variants
            .iter()
            .flat_map(|var| var.values.iter())
            .copied()
            .collect::<Vec<_>>(),
    )
    .map_err(|cause| {
        let who = variants
            .iter()
            .find(|var| var.values.iter().any(|val| val == &cause))
            .unwrap();

        Error::new(
            who.name.span(),
            format!("variant overlaps with a previous variant (at {cause})"),
        )
    })
}

fn check_coverage(values: &Values, max: u128, name: &Ident) -> Result<()> {
    match values.bounds() {
        Ok(bounds) if 0 < bounds.start => Err(Error::new(
            name.span(),
            format!("enum has uncovered case: ..{}", bounds.start),
        )),
        Ok(bounds) if bounds.end < max => Err(Error::new(
            name.span(),
            format!("enum has uncovered case: {}..{max}", bounds.end + 1),
        )),
        Err(gaps) => Err(Error::new(
            name.span(),
            format!("enum has uncovered case: {}", Into::<Values>::into(gaps)),
        )),

        Ok(_bounds) => Ok(()),
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
            .map(std::convert::TryInto::try_into)
            .collect::<Result<Vec<_>>>()?;
        if variants.is_empty() {
            return Err(Error::new(
                ast.name.span(),
                "zero-variant enum is not allowed",
            ));
        }

        check_range(variants.iter(), max)?;

        let has_default = variants.iter().any(|var| var.values.is_empty());

        let is_sealed = if has_default {
            false
        } else {
            let bounds = merge_values(&variants)?;
            check_coverage(&bounds, max, &ast.name)?;
            true
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
