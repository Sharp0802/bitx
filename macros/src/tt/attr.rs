use crate::prelude::*;
use crate::tt::{Error, Input, Parse, Token};

#[derive(Debug)]
pub struct Attr(TokenStream);

impl Parse for Attr {
    fn parse(input: &mut Input) -> Result<Self, Error> {
        let mut attr = TokenStream::new();

        loop {
            if !is!(input.peek(); Punct '#') {
                break;
            }

            let pound = input.pop();
            pound.to_tokens(&mut attr);

            let group = tok! {
                input.pop();

                Group @ group => group,
                _ => return Err(input.error("`[` expected")),
            };

            group.to_tokens(&mut attr);
        }

        Ok(Self(attr))
    }
}

impl ToTokens for Attr {
    fn to_tokens(&self, to: &mut TokenStream) {
        to.extend(self.0.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attr() {
        let ts = quote!(#[derive(Debug, Clone)]);
        let mut input: Input = ts.clone().into();
        let attr: Attr = input.parse().unwrap();

        assert_eq!(attr.to_token_stream().to_string(), ts.to_string());
    }

    #[test]
    fn test_attrs() {
        let ts = quote! {
            #[derive(Debug)]
            #[inline(always)]
            #[cfg(target_os = "linux")]
        };
        let mut input: Input = ts.clone().into();
        let attr: Attr = input.parse().unwrap();

        assert_eq!(attr.to_token_stream().to_string(), ts.to_string());
    }

    #[test]
    fn test_no_attr() {
        let ts = quote!(pub struct MyStruct);
        let mut input: Input = ts.into();
        let attr: Attr = input.parse().unwrap();

        assert!(attr.to_token_stream().is_empty());
        assert!(is!(input.peek(); Ident "pub"));
    }

    #[test]
    fn test_malformed() {
        let ts: TokenStream = "# pub struct".parse().unwrap();
        let mut input: Input = ts.into();

        assert!(input.parse::<Attr>().is_err());
    }
}
