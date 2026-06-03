use crate::ast::Values;
use crate::cg::to_tokens;
use crate::prelude::*;

to_tokens!(for Values; |self, tokens| {
    let iter = self.iter();
    tokens.extend(quote!{ ( #(#iter)|* ) });
});
