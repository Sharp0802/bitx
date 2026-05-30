use proc_macro2::{Span, TokenStream, Literal};
use quote::{quote, ToTokens};
use syn::parse::ParseStream;
use syn::{Attribute, Ident, Token, Visibility};

use crate::lit;
use crate::off::Offset;

#[inline]
fn read_mask(
    off_bytes: impl ToTokens,
    read_bytes: impl ToTokens,
    mask_bytes: impl ToTokens,
    mask: &syn::Type
) -> TokenStream {
    quote! {
        {
            let from = self.0
                .split_at(#off_bytes).1
                .split_at(#read_bytes).0;
            let mut buffer = [0u8; #mask_bytes];
            buffer
                .split_at_mut(#mask_bytes - #read_bytes).1
                .copy_from_slice(from);
            <#mask>::from_be_bytes(buffer)
        }
    }
}

pub enum Type {
    Literal {
        bits: usize,
        mask_bits: usize,
        mask: syn::Type,
    },
    Aligned(syn::Type),
    Unaligned(syn::Type),
}

impl Type {
    pub fn parse(
        input: ParseStream,
        name: &Ident,
        offset: Offset,
        bound: Offset,
    ) -> syn::Result<Self> {
        let raw: syn::Type = input.parse()?;
        
        // NOTE: custom nested type cannot be check at here;
        //       a detour using `Bits` trait is required.
        //       See `Field::assert`.
        
        if let Some(bits) = lit::size_of(&raw) {
            let total = offset.offset_bit(bits);

            if total > bound {
                return Err(input.error(format!(
                    "field `{name}` exceeds struct bounds",
                )));
            }

            let bytes = total.bits().div_ceil(8);
            let mask_bytes = bytes.next_power_of_two();
            let mask_bits = mask_bytes * 8;

            let Some(mask) = lit::with_bits(mask_bits) else {
                return Err(input.error(format!(
                    "unaligned field `{name}` cannot be \
                     larger than 128 bits",
                )));
            };

            Ok(Self::Literal {
                bits,
                mask_bits,
                mask,
            })
        } else if offset.bit == 0 {
            Ok(Self::Aligned(raw))
        } else {
            Ok(Self::Unaligned(raw))
        }
    }

    pub fn ret_ty(&self) -> syn::Type {
        match self {
            Self::Literal { bits: 1, .. } => lit::ty("bool"),
            Self::Literal { mask, .. } => mask.clone(),
            Self::Aligned(ty) | Self::Unaligned(ty) => ty.clone(),
        }
    }

    pub fn reader(&self, offset: Offset) -> TokenStream {
        const E_BIG_UNALIGNED: &str =
            "unaligned nested types cannot exceed 128 bits";
        
        match self {
            Self::Literal {
                bits,
                mask_bits,
                mask,
            } => {
                let off_bytes = offset.byte;
                let upper_bound_bits = offset.bit + bits;
                let read_bytes = upper_bound_bits.div_ceil(8);
                let mask_bytes = mask_bits / 8;

                let lpad_bits = read_bytes * 8 - upper_bound_bits;
                let rpad_bits = mask_bits - bits;

                let lpad = Literal::usize_unsuffixed(lpad_bits);
                let rpad = Literal::usize_unsuffixed(rpad_bits);
                
                let epilogue = if *bits == 1 {
                    quote! { val == 1 }
                } else {
                    quote! { val }
                };

                let read_stub = read_mask(
                    off_bytes,
                    read_bytes,
                    mask_bytes,
                    mask,
                );

                quote! {
                    let mut val = #read_stub;
                    val >>= #lpad;
                    val &= #mask::MAX >> #rpad;
                    #epilogue
                }
            }
            Self::Aligned(ty) => {
                let off_byte = offset.byte;
                let off_bit = Literal::usize_unsuffixed(offset.bit);

                let read_stub = read_mask(
                    off_byte,
                    Ident::new("SIZE", Span::call_site()),
                    16usize,
                    &lit::ty("u128")
                );

                let t32 = lit::ty("u32");
                let tsize = lit::ty("usize");
                quote! {
                    type Mask = <#ty as ::bitx::Bits>::Mask;

                    const BITS: #t32 = <#ty as ::bitx::Bits>::BITS;
                    const MASK: Mask = Mask::MAX>>(Mask::BITS - BITS);
                    const SIZE: #tsize =
                        (#off_bit + BITS as #tsize).div_ceil(8);

                    const _: () = assert!(
                        BITS % 8 == 0 || SIZE <= 16,
                        #E_BIG_UNALIGNED,
                    );

                    if BITS % 8 == 0 {
                        let from = self.0
                            .split_at(#off_byte).1
                            .split_at(SIZE).0;
                        
                        let mut buf = [0u8; SIZE];
                        buf.copy_from_slice(from);
                        
                        #ty::from_array(buf)
                    } else {
                        let mut val = #read_stub;
                        val >>= (SIZE as #t32 * 8) - (#off_bit + BITS);
                        let val = (val as Mask) & MASK;

                        #ty::__from_mask(val)
                    }
                }
            }
            Self::Unaligned(ty) => {
                let off_byte = offset.byte;
                let off_bit = Literal::usize_unsuffixed(offset.bit);

                let read_stub = read_mask(
                    off_byte,
                    Ident::new("SIZE", Span::call_site()),
                    16usize,
                    &lit::ty("u128"),
                );

                let t32 = lit::ty("u32");
                let tsize = lit::ty("usize");
                quote! {
                    type Mask = <#ty as ::bitx::Bits>::Mask;

                    const BITS: #t32 = <#ty as ::bitx::Bits>::BITS;
                    const MASK: Mask = Mask::MAX>>(Mask::BITS - BITS);
                    const SIZE: #tsize =
                        (#off_bit + BITS as #tsize).div_ceil(8);
                    
                    const _: () = assert!(
                        SIZE <= 16,
                        #E_BIG_UNALIGNED
                    );

                    let mut val = #read_stub;
                    val >>= (SIZE as #t32 * 8) - (#off_bit + BITS);
                    let val = (val as Mask) & MASK;

                    #ty::__from_mask(val)
                }
            }
        }
    }

} 


pub struct Field {
    attrs: Vec<Attribute>,
    offset: Offset,
    vis: Visibility,
    name: Ident,
    ty: Type,
}

impl Field {
    pub fn parse(
        input: ParseStream,
        bound: Offset,
    ) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let offset: Offset = input.parse()?;

        let vis: Visibility = input.parse()?;
        let name: Ident = input.parse()?;

        if offset >= bound {
            return Err(input.error(format!(
                "offset of field `{name}` exceeds struct bounds",
            )));
        }

        let _ = input.parse::<Token![:]>()?;

        let ty: Type = Type::parse(
            input,
            &name,
            offset,
            bound
        )?;

        Ok(Self {
            attrs,
            offset,
            vis,
            name,
            ty,
        })
    }

    pub fn assert(&self, size: Offset, stream: &mut TokenStream) {
        if let Type::Literal { .. } = &self.ty {
            // NOTE: already checked before when it parsed
            return;
        }
        
        let err = format!(
            "field `{}` exceeds struct bounds",
            self.name,
        );

        // SAFETY: `size` always greater than `off`; See `parse`.
        let max = Literal::usize_unsuffixed(
            size.bits() - self.offset.bits()
        );
        
        let ty = self.ty.ret_ty();

        stream.extend(quote! {
            assert!(<#ty as ::bitx::Bits>::BITS <= #max, #err);
        });
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.to_token_stream());
    }

    fn to_token_stream(&self) -> TokenStream {
        let attrs = &self.attrs;
        let name = &self.name;
        let vis = &self.vis;
        
        let stub = self.ty.reader(self.offset);
        let ret_ty = self.ty.ret_ty();

        quote! {
            #(#attrs)*
            #vis const fn #name(&self) -> #ret_ty {
                #stub
            }
        }
    }
}

