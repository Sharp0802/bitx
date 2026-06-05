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

    tst!(Visibility {
        pub: "pub",
        crate: "pub(crate)",
        super: "pub(super)",
        self: "pub(self)",
        priv: "",
    });
}
