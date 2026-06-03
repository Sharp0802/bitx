use crate::cg::to_tokens;
use crate::hir::{Mask, Struct};
use crate::prelude::*;
use crate::tt::Type;

to_tokens!(for Struct; |self, tokens| {
    let attr = &self.attr;
    let name = &self.name;
    let vis = &self.vis;
    let fields = self.fields.iter();

    // NOTE: Cannot default mask to types other than integers;
    //       Defaulting to those types gives users really annoying errors.
    //
    //       While unit type is used as default mask until version 0.2.2,
    //       It causes too many errors only for it's not valid integer type,
    //       not for that struct is too large to use mask.
    let mask = self.mask.clone().unwrap_or_else(|| Mask::new(128).unwrap());

    let mask_ty = mask.ty;
    let mask_bytes = Literal::u32_unsuffixed(mask.size / 8);
    let size = Literal::u32_unsuffixed(self.size);
    let bytes = Literal::u32_unsuffixed(self.size.div_ceil(8));

    let t8 = Type::literal(8);

    let mut assert = TokenStream::new();
    for field in &self.fields {
        field.assert(&mut assert);
    }

    let quoted = quote! {
        #attr
        #[repr(C, packed)]
        #vis struct #name([#t8; $bytes]);

        const _: () = { #assert };

        impl ::bitx::Bits for #name {
            type Mask = #mask_ty;
            const BITS: ::core::primitive::u32 = #size;
        }

        impl #name {
            #[inline]
            #[doc(hidden)]
            #vis const unsafe fn __from_mask(mask: #mask_ty) -> Self{
                let bytes = mask.to_be_bytes();
                // SAFETY: If and only if mask is None, bytes > mask_bytes.
                //         Thus this code will cause panic for large objects.
                //         As follows, `__from_mask` should be marked as unsafe.
                //         Safety of this function must be ensured from caller
                //         (assertion of .mask.is_some() == true).
                //
                // NOTE:   Note that we cannot ensure this condition at here,
                //         because we decided to default mask into `u128`
                //         to suppress compilation errors unrelated to
                //         direct reason.
                //
                //         See comment for `mask` variable also.
                let from = bytes.split_at(#mask_bytes - #bytes).1;

                let mut buf = [0u8; #bytes];
                buf.copy_from_slice(from);

                Self(buf)
            }

            #[inline]
            #vis const fn from_array(v: [#t8; #bytes]) -> Self {
                Self(v)
            }

            #[inline]
            #vis const fn from_slice(v: &[#t8])
                -> ::core::option::Option<&Self>
            {
                let Some((v, _)) = v.split_at_checked(#bytes) else {
                    return ::core::option::Option::None;
                };

                // SAFETY: 1. align is enforced to 1
                //         2. sizes are matched
                ::core::option::Option::Some(unsafe {
                    &*v.as_ptr().cast()
                })
            }

            #(#fields)*
        }
    };

    tokens.extend(quoted);
});
