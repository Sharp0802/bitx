use crate::hir::Layout;
use crate::prelude::*;
use crate::tt::Type;

impl Layout {
    pub fn quote_read(
        &self,
        ty: &Type,
        builtin: bool,
        tokens: &mut TokenStream,
    ) {
        let read_offset = self.read_offset_bytes;
        let read_bytes = self.read_bytes;

        if self.aligned {
            tokens.extend(quote! {
                let from = self.0
                    .split_at(#read_offset).1
                    .split_at(#read_bytes).0;
            });

            let cvt = if builtin {
                quote! {
                    let mut buffer = [0u8; #read_bytes];
                    ::bitx::copy(&mut buffer, from);

                    <#ty>::from_be_bytes(buffer)
                }
            } else {
                quote! { <#ty>::from_slice(from) }
            };

            tokens.extend(cvt);
        } else if let Some(mask) = &self.mask {
            let size = Literal::u32_unsuffixed(self.size);
            let mask_size = Literal::u32_unsuffixed(mask.size);
            let mask_bytes = Literal::u32_unsuffixed(mask.size / 8);
            let shr = Literal::u32_unsuffixed(self.shr);

            let mask = &mask.ty;

            tokens.extend(quote! {
                let from = self.0
                    .split_at(#read_offset).1
                    .split_at(#read_bytes).0;

                let mut buffer = [0u8; #mask_bytes];
                let into = buffer.split_at_mut(#mask_bytes - #read_bytes).1;
                ::bitx::copy(into, from);

                let mut val = <#mask>::from_be_bytes(buffer);
                val >>= #shr;
                val &= <#mask>::MAX >> (#mask_size - #size);
            });

            let cvt = if !builtin {
                quote! {
                    let val = val as <#ty as ::bitx::Bits>::Mask;
                    <#ty>::__from_mask(val)
                }
            } else if self.size > 1 {
                quote! { val as #ty }
            } else {
                quote! { val == 1 }
            };

            tokens.extend(cvt);
        } else {
            // TODO
            tokens.extend(quote! {
                ::core::compile_error!(
                    "unaligned large custom types are not supported yet"
                )
            });
        }
    }
}
