use crate::cg::to_tokens;
use crate::hir::Field;
use crate::prelude::*;

impl Field {
    pub fn assert(&self, tokens: &mut TokenStream) {
        if self.builtin {
            return;
        }

        let ty = &self.ty;
        let size = Literal::u32_unsuffixed(self.layout.size);

        let err = format!(
            "field `{}` has incompatible size with its type",
            self.name,
        );

        tokens.extend(quote! {
            ::core::assert!(<#ty as ::bitx::Bits>::BITS == #size, #err);
        });
    }
}

to_tokens!(for Field; |self, tokens| {
    let name = &self.name;
    let vis = &self.vis;
    let attr = &self.attr;
    let ty = &self.ty;

    let mut body = TokenStream::new();
    self.assert(&mut body);
    self.layout.quote_read(ty, self.builtin, &mut body);

    let mut ret_ty = TokenStream::new();
    self.layout.return_type(ty, self.builtin, &mut ret_ty);

    let getter = quote! {
        #attr
        #[inline]
        #vis const fn #name(&self) -> #ret_ty {
            #body
        }
    };

    tokens.extend(getter);
});
