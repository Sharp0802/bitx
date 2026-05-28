use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_quote, Attribute, Ident, Token, Type, Visibility};

use crate::lit;
use crate::off::Offset;

pub struct Field {
    pub attrs: Vec<Attribute>,
    pub off: Offset,
    pub vis: Visibility,
    pub name: Ident,
    pub ty: Type,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream());
    }

    fn to_token_stream(&self) -> TokenStream {
        let off = self.off.byte;
        let bit_off = self.off.bit;

        let attrs = &self.attrs;
        let name = &self.name;
        let vis = &self.vis;
        let ty = &self.ty;

        let t32: Type = parse_quote!(::core::primitive::u32);
        let t128: Type = parse_quote!(::core::primitive::u128);
        let tsize: Type = parse_quote!(::core::primitive::usize);

        if let Some(bits) = lit::size_of(&self.ty) {
            // NOTE: if type is literal (u1, u2, ..., u128).
            let size = (bit_off + bits).div_ceil(8);
            let base = size.next_power_of_two();

            let Some(ty) = lit::with_bits(base * 8) else {
                return quote! {
                    compile_error!(
                        "unaligned extraction is not supported \
                         for sizes larger than 128-bit"
                    )
                };
            };

            let shr = u32::try_from(size * 8 - bit_off - bits).unwrap();
            let mask_shr = u32::try_from(base * 8 - bits).unwrap();

            let ret: Type = if bits == 1 {
                parse_quote!(::core::primitive::bool)
            } else {
                ty.clone()
            };

            let epi = if bits == 1 {
                quote! { val == 1 }
            } else {
                quote! { val }
            };

            quote! {
                #(#attrs)*
                #vis const fn #name(&self) -> #ret {
                    let mut val = {
                        let mut buf = [0u8; #base];

                        // let from = self.0[#off..(#off+#size)]
                        let from = self.0
                            .split_at(#off).1
                            .split_at(#size).0;

                        // buf[(#base-#size)..].copy_from_slice(...)
                        buf
                            .split_at_mut(#base - #size).1
                            .copy_from_slice(from);

                        <#ty>::from_be_bytes(buf)
                    };
                    val >>= #shr;
                    val &= #ty::MAX >> #mask_shr;

                    #epi
                }
            }
        } else if bit_off == 0 {
            // NOTE: or it's aligned.
            quote! {
                #(#attrs)*
                #vis const fn #name(self) -> #ty {
                    const BITS: #t32   = <#ty as ::bitx::Bits>::BITS;
                    const SIZE: #tsize = (BITS as #tsize + 7) / 8;

                    unsafe {
                        self.0.as_ptr()
                            .add(#off)
                            .cast::<#ty>()
                            .read_unaligned()
                    }
                }
            }
        } else {
            let bit_off32 = u32::try_from(bit_off).unwrap();

            // NOTE: otherwise: must be smaller than 128-bit.
            quote! {
                #(#attrs)*
                #vis const fn #name(&self) -> #ty {
                    type M = <#ty as ::bitx::Bits>::Mask;

                    const BITS  : #t32   = <#ty as ::bitx::Bits>::BITS;
                    const SIZE32: #t32   = (#bit_off32 + BITS + 7) / 8;
                    const SIZE  : #tsize = SIZE32 as #tsize;

                    const MASK: M = M::MAX >> (M::BITS - BITS);

                    const _: () = assert!(
                        SIZE <= 16,
                        "unaligned nested types cannot exceed 128-bit",
                    );

                    let mut val: #t128 = {
                        let mut buf = [0u8; 16usize];

                        // let from = self.0[#off..(#off+SIZE)]
                        let from = self.0
                            .split_at(#off).1
                            .split_at(SIZE).0;

                        // buf[(16-SIZE)..].copy_from_slice(..)
                        buf
                            .split_at_mut(16 - SIZE).1
                            .copy_from_slice(from);

                        #t128::from_be_bytes(buf)
                    };
                    val >>= (SIZE32 * 8) - (#bit_off32 + BITS);
                    let val = (val as M) & MASK;

                    #ty::__from_mask(val)
                }
            }
        }
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let off: Offset = input.parse()?;
        let vis: Visibility = input.parse()?;
        let name: Ident = input.parse()?;

        let _ = input.parse::<Token![:]>()?;

        let ty: Type = input.parse()?;

        Ok(Self {
            attrs,
            off,
            vis,
            name,
            ty,
        })
    }
}
