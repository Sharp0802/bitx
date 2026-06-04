use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};

#[derive(Debug)]
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

    roundtrip!(roundtrip_pub "pub" |val: Visibility| {
        assert!(val.public);
        assert!(val.inner.is_none());
    });

    roundtrip!(roundtrip_pub_crate "pub(crate)" |val: Visibility| {
        assert!(val.public);
        assert!(val.inner.is_some());
    });

    roundtrip!(roundtrip_pub_super "pub(super)" |val: Visibility| {
        assert!(val.public);
        assert!(val.inner.is_some());
    });

    roundtrip!(roundtrip_pub_self "pub(self)" |val: Visibility| {
        assert!(val.public);
        assert!(val.inner.is_some());
    });

    roundtrip!(roundtrip_priv "" |val: Visibility| {
        assert!(!val.public);
        assert!(val.inner.is_none());
    });
}
