use crate::cg::to_tokens;
use crate::hir::Field;
use crate::prelude::*;

impl Field {
    pub fn assert(&self, tokens: &mut TokenStream) {
        let ty = &self.ty;
        let size = Literal::u32_unsuffixed(self.layout.size);
        let offset = Literal::u32_unsuffixed(self.layout.offset);

        let err = format!(
            "field `{}` has incompatible size ({}-bit) with its type ({{}}-bit)",
            self.name, self.layout.size,
        );

        tokens.extend(quote! {
            ::core::assert!(
                #offset + <#ty as ::bitx::Bits>::BITS <= #size,
                #err,
                <#ty as ::bitx::Bits>::BITS,
            );
        });
    }
}

to_tokens!(for Field; |self, tokens| {
    let name = &self.name;
    let vis = &self.vis;
    let attr = &self.attr;

    let mut body = TokenStream::new();
    self.assert(&mut body);
    self.layout.quote_read(&self.ty, self.builtin, &mut body);

    let getter = quote! {
        #attr
        #[inline]
        #vis const fn #name(&self) {
            #body
        }
    };

    tokens.extend(getter);
});
