use crate::field::Field;
use crate::lit;
use crate::off::Offset;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_quote, Attribute, Ident, Token, Type, Visibility};

pub struct Struct {
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    fields: Punctuated<Field, Token![,]>,
    bits: usize,
}

impl ToTokens for Struct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let bits = self.bits;
        let size = bits.div_ceil(8);

        let bits32 = u32::try_from(bits).unwrap();

        let fields = self.fields.iter();
        let attrs = &self.attrs;
        let name = &self.name;
        let vis = &self.vis;

        let hash = quote! { # };
        let t8: Type = parse_quote!(::core::primitive::u8);
        let t32: Type = parse_quote!(::core::primitive::u32);
        let tsize: Type = parse_quote!(::core::primitive::usize);

        // - DEFINITION ------
        tokens.extend(quote! {
            #(#attrs)*
            #hash [repr(C)]
            #vis struct #name([#t8; #size]);

            impl #name {
                pub const fn from_array(value: [#t8; #size]) -> Self {
                    Self(value)
                }

                pub const fn from_slice(value: &[#t8])
                    -> ::core::option::Option<&Self>
                {
                    let Some((v, _)) = value.split_at_checked(#size)
                    else {
                        return None;
                    };

                    // SAFETY: 1. align is enforced to 1
                    //         2. sizes are matched
                    Some(unsafe {
                        &*v.as_ptr().cast()
                    })
                }

                #(#fields)*
            }
        });

        // - MASK ------
        let mask_size = size.next_power_of_two();
        let masks = lit::with_bits(mask_size * 8).map_or_else(
            || quote! {
                impl ::bitx::Bits for #name {
                    type Mask = ();

                    const BITS: #t32 = #bits32;
                }
            },
            |mask| quote! {
                impl ::bitx::Bits for #name {
                    type Mask = #mask;
                    
                    const BITS: #t32 = #bits32;
                }
                
                impl #name {
                    const fn __from_mask(mask: #mask) -> Self { 
                        let mut buf = [0u8; #size];

                        // NOTE: const index is not yet stable
                        let bytes = mask.to_be_bytes();
                        let from = bytes.split_at(#mask_size-#size).1;

                        buf.copy_from_slice(&from);
                        Self(buf)
                    }
                }
            }
        );
        tokens.extend(masks);

        // - VALIDATION ------
        for field in &self.fields {
            let ty = &field.ty;
            let off = field.off.byte * 8 + field.off.bit;
            let name = field.name.to_string();

            let ty_bits = lit::size_of(ty).map_or_else(
                || quote! { <#ty as ::bitx::Bits>::BITS as #tsize },
                |bits| quote! { #bits },
            );

            let err = format!("'{name}' exceeds struct bounds");
            tokens.extend(quote! {
                const _: () = assert!(#off + #ty_bits <= #bits, #err);
            });
        }
    }
}

impl Parse for Struct {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let vis: Visibility = input.parse()?;
        let _ = input.parse::<Token![struct]>()?;
        let name: Ident = input.parse()?;
        let _ = input.parse::<Token![:]>()?;
        let bits = {
            let off: Offset = input.parse()?;
            off.byte * 8 + off.bit
        };

        let braced;
        syn::braced!(braced in input);

        let fields =
            braced.parse_terminated(Field::parse, Token![,])?;

        Ok(Self {
            attrs,
            vis,
            name,
            fields,
            bits,
        })
    }
}
