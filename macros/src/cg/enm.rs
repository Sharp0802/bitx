use crate::ast::Variant;
use crate::cg::to_tokens;
use crate::hir::Enum;
use crate::prelude::*;
use crate::tt::Type;

to_tokens!(for Enum; |self, tokens| {
    let attr = &self.attr;
    let vis = &self.vis;
    let name = &self.name;
    let mask_ty = &self.mask.ty;

    let size = Literal::u32_unsuffixed(self.size);
    let bytes = Literal::u32_unsuffixed(self.size.div_ceil(8));
    let mask_size = Literal::u32_unsuffixed(self.mask.size);
    let mask_bytes = Literal::u32_unsuffixed(self.mask.size / 8);
    let repr = Ident::new(&format!("u{mask_size}"), Span::call_site());

    let defs = self.variants.iter().map(|var| var.quote_def(mask_ty));
    let arms = self.variants.iter().map(Variant::quote_arm);

    let t8 = Type::literal(8);

    let expanded = quote! {
        #attr
        #[derive(Debug, Copy, Clone, PartialEq, Eq)]
        #[repr(C, #repr)]
        #vis enum #name {
            #(#defs),*
        }

        impl ::bitx::Bits for #name {
            type Mask = #mask_ty;
            const BITS: ::core::primitive::u32 = #size;
        }

        impl #name {
            #[inline]
            #[doc(hidden)]
            #vis const fn __from_mask(val: #mask_ty) -> Self {
                match val & (#mask_ty::MAX >> (#mask_size - #size)) {
                    #(#arms,)*
                    _ => unreachable!(),
                }
            }

            #[inline]
            #vis const fn from_array(val: [#t8; #bytes]) -> Self {
                let mut buffer = [0u8; #mask_bytes];
                buffer
                    .split_at_mut(#mask_bytes - #bytes).1
                    .copy_from_slice(&val);
                Self::__from_mask(#mask_ty::from_be_bytes(buffer))
            }

            #[inline]
            #vis const fn from_slice(val: &[#t8])
                -> core::option::Option<Self>
            {
                let buffer = if val.len() >= #mask_bytes {
                    val.split_at(#mask_bytes).0.try_into().unwrap()
                } else if val.len() >= #bytes {
                    let mut buffer = [0u8; #mask_bytes];
                    buffer
                        .split_at_mut(#mask_bytes - #bytes).1
                        .copy_from_slice(val.split_at(#bytes).0);
                    buffer
                } else {
                    return core::option::Option::None;
                };

                core::option::Option::Some(
                    Self::__from_mask(#mask_ty::from_be_bytes(buffer))
                )
            }
        }
    };

    tokens.extend(expanded);
});
