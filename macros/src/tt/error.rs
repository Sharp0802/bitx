use crate::prelude::*;

#[derive(Debug)]
pub struct Error {
    message: &'static str,
    span: Span,
}

impl Error {
    pub fn new(message: &'static str, span: Span) -> Self {
        Self { message, span }
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, to: &mut TokenStream) {
        let msg = self.message;
        
        to.extend(
            quote_spanned! { self.span =>
                ::core::compile_error!(#msg)
            }
        )
    }
}
