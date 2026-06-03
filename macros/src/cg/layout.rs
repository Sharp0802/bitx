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
        let read_size = self.read_bytes;

        if self.aligned {
            tokens.extend(quote! {
                let from = self.0
                    .split_at(#read_offset).1
                    .split_at(#read_size).0;
            });

            let cvt = if builtin {
                quote! {
                    let mut buffer = [0u8; #read_size];
                    buffer.copy_from_slice(from);

                    <#ty>::from_be_bytes(buffer)
                }
            } else {
                quote! { <#ty>::from_slice(from) }
            };

            tokens.extend(cvt);
        } else if let Some(mask) = &self.mask {
            let size = self.size;
            let mask_size = mask.size;
            let mask = &mask.ty;
            let shr = self.shr;

            tokens.extend(quote! {
                let from = self.0
                    .split_at(#read_offset).1
                    .split_at(#read_size).0;

                let buffer = [0u8; #read_size];
                buffer.copy_from_slice(from);

                let mut val = <#mask>::from_be_bytes(buffer);
                val >>= #shr;
                val &= <#mask>::MAX >> (#mask_size - #size);
            });

            let cvt = if !builtin {
                quote! { <#ty>::__from_mask(val) }
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
                    "unaligned nested custom type is not supported yet"
                )
            });
        }
    }
}
