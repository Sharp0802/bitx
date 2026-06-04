use crate::ast::{Body, Data, Values, Variant};
use crate::hir::Mask;
use crate::prelude::*;
use crate::tt::{Attr, Block, Error, Visibility};

#[derive(Debug)]
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

    #[inline(always)]
    #[expect(
        clippy::inline_always,
        reason = "1. fn is used only at `hir::Data::try_from`.\n\
                  2. prologue checks can be omitted by context.\n\
                  3. inlining may not be done without `always`."
    )]
    fn try_from(ast: Data) -> Result<Self, Error> {
        // NOTE: this check should be omitted by inlining
        let Body::Enum(variants) = ast.body else {
            return Err(Error::new(
                "internal: cannot raising struct AST into enum HIR",
                ast.name.span(),
            ));
        };

        let name = ast.name;
        let size = ast.size;

        let mask = Mask::new(size).ok_or_else(|| {
            Error::new("enum cannot be larger than 128 bits", name.span())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Data;
    use crate::tt::Input;

    fn parse(ts: TokenStream) -> Data {
        let mut input: Input = ts.into();
        input.parse().unwrap()
    }

    fn assert_err(data: Data, needle: &str) {
        let err = Enum::try_from(data).unwrap_err();
        assert!(err.message().contains(needle));
    }

    #[test]
    fn raise_struct_as_enum() {
        let mut input: Input = quote!(struct Foo: u1 {}).into();
        let data: Data = input.parse().unwrap();
        assert!(TryInto::<Enum>::try_into(data).is_err());
    }

    #[test]
    fn full_coverage() {
        let data = parse(quote! {
            enum E: u2 {
                0 => A,
                1 => B,
                2 => C,
                3 => D,
            }
        });
        _ = Enum::try_from(data).unwrap();
    }

    #[test]
    fn default_allows_gap() {
        let data = parse(quote! {
            enum E: u4 {
                0..=2 => A,
                3 => B,
                _ => Rest,
            }
        });
        _ = Enum::try_from(data).unwrap();
    }

    #[test]
    fn conflict_default() {
        let data = parse(quote! {
            enum E: u2 {
                0 => A,
                _ => B,
                _ => C,
            }
        });
        assert_err(data, "conflict default");
    }

    #[test]
    fn zero_variants() {
        let data = parse(quote!(enum E: u2 {}));
        assert_err(data, "zero-variant");
    }

    #[test]
    fn too_big() {
        let data = parse(quote!(enum E: u129 { 0 => A }));
        assert_err(data, "larger than 128");
    }

    #[test]
    fn overlap() {
        let data = parse(quote! {
            enum E: u3 {
                0..=3 => A,
                2..=4 => B,
            }
        });
        assert_err(data, "overlap");
    }

    #[test]
    fn value_exceeds_max() {
        let data = parse(quote!(enum E: u2 { 5 => A }));
        assert_err(data, "exceeds");
    }

    #[test]
    fn range_exceeds_max() {
        let data = parse(quote!(enum E: u2 { 0..=4 => A }));
        assert_err(data, "exceeds");
    }

    #[test]
    fn coverage_gap_below() {
        let data = parse(quote! {
            enum E: u3 {
                2..=2 => A,
                3..=3 => B,
                4..=4 => C,
            }
        });
        assert_err(data, "uncovered case");
    }

    #[test]
    fn coverage_gap_above() {
        let data = parse(quote! {
            enum E: u3 {
                0 => A,
                1 => B,
                2 => C,
            }
        });
        assert_err(data, "uncovered case");
    }

    #[test]
    fn coverage_gap_middle() {
        let data = parse(quote! {
            enum E: u4 {
                0..=1 => A,
                4..=7 => B,
            }
        });
        assert_err(data, "uncovered case");
    }
}
