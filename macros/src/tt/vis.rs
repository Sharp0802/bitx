use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};

pub struct Visibility {
    public: bool,
    inner: Option<Token>,
}

impl Parse for Visibility {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        tok! {
            input.peek();

            Ident "pub" => {
                _ = input.pop();
            },
            _ => return Ok(Self { public: false, inner: None }),
        }

        let group = tok! {
            input.peek();

            Group @ group => input.pop(),
            _ => return Ok(Self { public: true, inner: None }),
        };

        Ok(Self {
            public: true,
            inner: Some(group),
        })
    }
}

impl ToTokens for Visibility {
    fn to_tokens(&self, to: &mut TokenStream) {
        let quoted = if let Some(inner) = &self.inner {
            quote!(pub #inner)
        } else if self.public {
            quote!(pub)
        } else {
            return;
        };

        to.extend(quoted);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public() {
        let ts = quote!(pub);
        let mut input: Input = ts.into();
        let vis: Visibility = input.parse().unwrap();

        assert!(vis.public);
        assert!(vis.inner.is_none());
    }

    #[test]
    fn test_restricted() {
        let ts = quote!(pub(crate));
        let mut input: Input = ts.into();
        let vis: Visibility = input.parse().unwrap();

        assert!(vis.public);
        assert!(vis.inner.is_some());
    }

    #[test]
    fn test_private() {
        let ts = quote!();
        let mut input: Input = ts.into();
        let vis: Visibility = input.parse().unwrap();

        assert!(!vis.public);
        assert!(vis.inner.is_none());
    }

    #[test]
    fn to_tokens_public() {
        // `pub` round-trips to `pub`.
        let mut input: Input = quote!(pub).into();
        let vis: Visibility = input.parse().unwrap();

        let mut out = TokenStream::new();
        vis.to_tokens(&mut out);
        assert_eq!(out.to_string(), "pub");
    }

    #[test]
    fn to_tokens_restricted() {
        // `pub(crate)` round-trips with the group preserved.
        let mut input: Input = quote!(pub(crate)).into();
        let vis: Visibility = input.parse().unwrap();

        let mut out = TokenStream::new();
        vis.to_tokens(&mut out);
        assert_eq!(out.to_string(), "pub (crate)");
    }

    #[test]
    fn to_tokens_private_emits_nothing() {
        // A missing `pub` should produce an empty token stream
        // (the early-return branch in `ToTokens`).
        let mut input: Input = quote!().into();
        let vis: Visibility = input.parse().unwrap();

        let mut out = TokenStream::new();
        vis.to_tokens(&mut out);
        assert!(out.is_empty());
    }

    #[test]
    fn to_tokens_pub_super() {
        // `pub(super)` is also valid restricted visibility.
        let mut input: Input = quote!(pub(super)).into();
        let vis: Visibility = input.parse().unwrap();

        let mut out = TokenStream::new();
        vis.to_tokens(&mut out);
        assert_eq!(out.to_string(), "pub (super)");
    }
}
