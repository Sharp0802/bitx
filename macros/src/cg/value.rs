use crate::ast::Value;
use crate::cg::to_tokens;
use crate::prelude::*;

to_tokens!(for Value; |self, tokens| {
    let start = Literal::u128_unsuffixed(self.start);
    let end = Literal::u128_unsuffixed(self.end);

    let quoted = if self.start == self.end {
        quote! { #start }
    } else {
        quote! { #start..=#end }
    };

    tokens.extend(quoted);
});
