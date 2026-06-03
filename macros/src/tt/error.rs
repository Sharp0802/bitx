use crate::prelude::*;
use std::borrow::Cow;

#[derive(Debug)]
pub struct Error {
    message: Cow<'static, str>,
    span: Span,
}

impl Error {
    pub fn new<M: Into<Cow<'static, str>>>(message: M, span: Span) -> Self {
        Self {
            message: message.into(),
            span,
        }
    }

    #[cfg(test)]
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl ToTokens for Error {
    fn to_tokens(&self, to: &mut TokenStream) {
        let msg = self.message.as_ref();

        to.extend(quote_spanned! { self.span =>
            ::core::compile_error!(#msg);
        });
    }
}
