use crate::ast::Variant;
use crate::prelude::*;
use crate::tt::Type;

impl Variant {
    pub fn quote_def(&self, mask: &Type) -> TokenStream {
        let attr = &self.attr;
        let name = &self.name;

        if self.values.is_point() {
            quote! { #attr #name }
        } else {
            quote! { #attr #name(#mask) }
        }
    }

    pub fn quote_arm(&self) -> TokenStream {
        let name = &self.name;

        if self.values.is_empty() {
            quote! { val => Self::#name(val) }
        } else if let Some(point) = self.values.point() {
            let val = Literal::u128_unsuffixed(point);
            quote! { #val => Self::#name }
        } else {
            let pat = &self.values;
            quote! { val @ #pat => Self::#name(val) }
        }
    }
}
