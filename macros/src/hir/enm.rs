use crate::ast::{Body, Data, Values, Variant};
use crate::hir::Mask;
use crate::prelude::*;
use crate::tt::{Attr, Block, Error, Visibility};

pub struct Enum {
    pub attr: Attr,
    pub vis: Visibility,
    pub name: Ident,
    pub size: u32,
    pub mask: Mask,
    pub variants: Block<Variant>,
}

fn check_range(variants: &Block<Variant>, max: u128) -> Result<(), Error> {
    for variant in variants.iter() {
        for value in variant.values.iter() {
            if value.end <= max {
                continue;
            }

            return Err(Error::new(
                format!(
                    "value of variant exceeds \
                        maximum value of enum (`{max}`)"
                ),
                variant.name.span(),
            ));
        }
    }

    Ok(())
}

fn merge_values(variants: &Block<Variant>) -> Result<Values, Error> {
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
            format!("variant overlaps with a previous variant (at {cause})"),
            who.name.span(),
        )
    })
}

fn check_coverage(
    variants: &Block<Variant>,
    max: u128,
    name: &Ident,
) -> Result<(), Error> {
    match merge_values(variants)?.bounds() {
        Ok(bounds) if 0 < bounds.start => Err(Error::new(
            format!("enum has uncovered case: ..{}", bounds.start),
            name.span(),
        )),
        Ok(bounds) if bounds.end < max => Err(Error::new(
            format!("enum has uncovered case: {}..={max}", bounds.end + 1),
            name.span(),
        )),
        Err(gaps) => Err(Error::new(
            format!("enum has uncovered case: {}", Into::<Values>::into(gaps)),
            name.span(),
        )),

        Ok(_bounds) => Ok(()),
    }
}

impl TryFrom<Data> for Enum {
    type Error = Error;

    fn try_from(ast: Data) -> Result<Self, Error> {
        let Body::Enum(variants) = ast.body else {
            panic!("enum expected");
        };

        let name = ast.name;
        let size = ast.size;

        let mask = Mask::new(size).ok_or_else(|| {
            if size > 0 {
                Error::new("enum cannot be larger than 128 bits", name.span())
            } else {
                Error::new("`u0` is not allowed", name.span())
            }
        })?;

        let max = if size == 128 {
            u128::MAX
        } else {
            (1u128 << size) - 1
        };

        if variants.is_empty() {
            return Err(Error::new(
                "zero-variant enum is not allowed",
                name.span(),
            ));
        }

        check_range(&variants, max)?;

        let mut defaults = variants.iter().filter(|var| var.values.is_empty());
        let has_default = defaults.next().is_some();

        if let Some(other) = defaults.next() {
            return Err(Error::new(
                "enum has conflict default",
                other.name.span(),
            ));
        }

        if !has_default {
            check_coverage(&variants, max, &name)?;
        }

        Ok(Self {
            attr: ast.attr,
            vis: ast.vis,
            name,
            size,
            mask,
            variants,
        })
    }
}
