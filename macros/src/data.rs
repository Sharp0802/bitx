use crate::field::Field;
use crate::lit;
use crate::off::Offset;
use crate::variant::Variant;
use proc_macro2::{Literal, Span, TokenStream};
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::token::Pound;
use syn::{Attribute, Ident, Token, Visibility};

pub enum Body {
    Struct(Punctuated<Field, Token![,]>),
    Enum {
        variants: Punctuated<Variant, Token![,]>,
        sealed: bool,
    },
}

pub struct Data {
    attrs: Vec<Attribute>,
    vis: Visibility,
    name: Ident,
    body: Body,
    size: Offset,
}

impl Data {
    fn stub_decl(&self) -> TokenStream {
        let attrs = &self.attrs;
        let name = &self.name;
        let vis = &self.vis;

        let sh = Pound::default();
        let t8 = lit::ty("u8");

        if let Body::Enum { variants, .. } = &self.body {
            let mask_bits =
                8 * self.size.bits().div_ceil(8).next_power_of_two();

            let repr =
                Ident::new(&format!("u{mask_bits}"), Span::call_site());

            let variants = variants.iter();
            quote! {
                #(#attrs)*
                #sh[derive(Copy, Clone, Eq, PartialEq)]
                #sh[repr(#repr)]
                #vis enum #name {
                    #(#variants),*
                }
            }
        } else {
            let size = self.size.bits().div_ceil(8);
            quote! {
                #(#attrs)*
                #sh[derive(Copy, Clone, Eq, PartialEq)]
                #sh[repr(C, packed)]
                #vis struct #name([#t8; #size]);
            }
        }
    }

    fn stub_gen_mask(&self) -> TokenStream {
        let name = &self.name;

        let bits = Literal::usize_unsuffixed(self.size.bits());

        let bytes = self.size.bits().div_ceil(8);

        let mask_bytes = bytes.next_power_of_two();
        let mask_bits = mask_bytes * 8;

        let t32 = lit::ty("u32");

        if let Body::Enum { variants, sealed } = &self.body {
            let mask = lit::with_bits(mask_bytes * 8).unwrap();

            let arms = variants
                .iter()
                .map(super::variant::Variant::to_match_arm);

            let last = if *sealed {
                Variant::unreachable()
            } else {
                TokenStream::new()
            };

            quote! {
                impl ::bitx::Bits for #name {
                    type Mask = #mask;
                    const BITS: #t32 = #bits;
                }

                impl #name {
                    pub const fn __from_mask(mask: #mask) -> Self {
                        match mask {
                            #(#arms,)*
                            #last
                        }
                    }
                }
            }
        } else {
            lit::with_bits(mask_bits).map_or_else(
                || {
                    quote! {
                        impl ::bitx::Bits for #name {
                            type Mask = ();
                            const BITS: #t32 = #bits;
                        }
                    }
                },
                |mask| {
                    quote! {
                        impl ::bitx::Bits for #name {
                            type Mask = #mask;
                            const BITS: #t32 = #bits;
                        }

                        impl #name {
                            const fn __from_mask(mask: #mask) -> Self {
                                let bytes = mask.to_be_bytes();
                                let from = bytes
                                    .split_at(#mask_bytes - #bytes).1;

                                let mut buf = [0u8; #bytes];
                                buf.copy_from_slice(&from);

                                Self(buf)
                            }
                        }
                    }
                },
            )
        }
    }

    fn stub_gen_slice(&self) -> TokenStream {
        let name = &self.name;

        let bytes = self.size.bits().div_ceil(8);

        let mask_bytes = bytes.next_power_of_two();

        let mask_bits = mask_bytes * 8;
        let bits = self.size.bits();
        let erase_bits = Literal::usize_unsuffixed(mask_bits - bits);

        let t8 = lit::ty("u8");

        if let Body::Enum { .. } = &self.body {
            quote! {
                impl #name {
                    pub const fn from_array(v: [#t8; #bytes]) -> Self {
                         Self::from_slice(&v).unwrap()
                    }

                    pub const fn from_slice(v: &[#t8])
                        -> ::core::option::Option<Self>
                    {
                        type Mask = <#name as ::bitx::Bits>::Mask;

                        let Some((v, _)) = v.split_at_checked(#bytes)
                        else {
                            return None;
                        };

                        let mut buf = [0u8; #mask_bytes];
                        buf
                            .split_at_mut(#mask_bytes - #bytes).1
                            .copy_from_slice(&v);

                        let mut mask = Mask::from_be_bytes(buf);
                        mask &= (Mask::MAX >> #erase_bits);

                        Some(Self::__from_mask(mask))
                    }
                }
            }
        } else {
            quote! {
                impl #name {
                    pub const fn from_array(v: [#t8; #bytes]) -> Self {
                        Self(v)
                    }

                    pub const fn from_slice(v: &[#t8])
                        -> ::core::option::Option<&Self>
                    {
                        let Some((v, _)) = v.split_at_checked(#bytes)
                        else {
                            return None;
                        };

                        // SAFETY: 1. align is enforced to 1
                        //         2. sizes are matched
                        Some(unsafe {
                            &*v.as_ptr().cast()
                        })
                    }
                }
            }
        }
    }

    fn stub_fields(&self) -> TokenStream {
        let Body::Struct(fields) = &self.body else {
            return TokenStream::new();
        };

        let name = &self.name;

        let mut checks = TokenStream::new();
        for field in fields {
            field.assert(self.size, &mut checks);
        }

        let fields = fields.iter();
        quote! {
            impl #name {
                #(#fields)*
            }

            const _: () = { #checks };
        }
    }
}

impl ToTokens for Data {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.stub_decl());
        tokens.extend(self.stub_gen_mask());
        tokens.extend(self.stub_gen_slice());
        tokens.extend(self.stub_fields());
    }
}

impl Parse for Data {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let vis: Visibility = input.parse()?;

        let is_enum = if input.peek(Token![enum]) {
            let _ = input.parse::<Token![enum]>()?;
            true
        } else {
            let _ = input.parse::<Token![struct]>()?;
            false
        };

        let name: Ident = input.parse()?;
        let _ = input.parse::<Token![:]>()?;
        let size: Offset = input.parse()?;

        if is_enum && size.bits() > 128 {
            return Err(
                input.error("enum cannot be larger than 128-bit")
            );
        }

        let content;
        syn::braced!(content in input);

        let body = if is_enum {
            let variants =
                content.parse_terminated(Variant::parse, Token![,])?;
            if variants.is_empty() {
                return Err(syn::Error::new(
                    name.span(),
                    "zero-variant enum is not allowed",
                ));
            }

            let req = size.bits();
            let max = if req == 128 {
                u128::MAX
            } else {
                (1u128 << req) - 1
            };

            let mut default: Option<&Ident> = None;
            for variant in &variants {
                if let Some(lit) = &variant.value {
                    let val: u128 = lit.base10_parse()?;
                    
                    if val <= max {
                        continue;
                    }
                        
                    return Err(syn::Error::new(
                        lit.span(),
                        format!(
                            "variant value `{val}` exceeds the maximum \
                             allowed value (`{max}`) for a {req}-bit \
                             enum"
                        ),
                    ));
                }

                if let Some(old) = &default {
                    return Err(syn::Error::new(
                        variant.name.span(),
                        format!(
                            "default is already defined at `{old}`",
                        ),
                    ));
                }
                default = Some(&variant.name);
            }

            let pow = variants.len().is_power_of_two();
            let cur = variants.len().ilog2() as usize;

            if cur > req || (!pow && cur == req) {
                return Err(syn::Error::new(
                    name.span(),
                    "enum is overstuffed",
                ));
            }

            let sealed = if default.is_none() {
                if req != cur {
                    return Err(syn::Error::new(
                        name.span(),
                        "enum has uncovered cases",
                    ));
                }

                true
            } else {
                false
            };

            Body::Enum { variants, sealed }
        } else {
            let mut fields = Punctuated::new();

            while !content.is_empty() {
                fields.push_value(Field::parse(&content, size)?);

                if content.is_empty() {
                    break;
                }

                fields.push_punct(content.parse::<Token![,]>()?);
            }

            Body::Struct(fields)
        };

        Ok(Self {
            attrs,
            vis,
            name,
            body,
            size,
        })
    }
}
