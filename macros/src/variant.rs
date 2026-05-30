use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Ident, LitInt, Token};

pub struct Variant {
    pub attrs: Vec<Attribute>,
    pub value: Option<LitInt>,
    pub name: Ident,
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attrs = &self.attrs;
        let name = &self.name;

        let def = self.value.as_ref().map_or_else(
            || quote! { #(#attrs)* #name },
            |val| quote! { #(#attrs)* #name = #val },
        );

        tokens.extend(def);
    }
}

impl Variant {
    pub fn to_match_arm(&self) -> TokenStream {
        let name = &self.name;

        self.value.as_ref().map_or_else(
            || quote! { _ => Self::#name },
            |val| quote! { #val => Self::#name },
        )
    }

    pub fn unreachable() -> TokenStream {
        quote! { _ => unreachable!() }
    }
}

impl Parse for Variant {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;

        let value = if input.peek(Token![_]) {
            let _ = input.parse::<Token![_]>()?;
            None
        } else {
            Some(input.parse::<LitInt>()?)
        };

        let name: Ident = input.parse()?;

        Ok(Self { attrs, value, name })
    }
}
