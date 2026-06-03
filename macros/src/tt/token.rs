use crate::prelude::*;
use crate::tt::{Error, Input, Parse};

#[derive(Debug, Clone)]
pub enum Token {
    Ident(Ident),
    Punct(Punct),
    Group(Group),
    Literal(Literal),
    End,
}

impl Token {
    pub fn span(&self) -> Span {
        match self {
            Self::Ident(val) => val.span(),
            Self::Punct(val) => val.span(),
            Self::Group(val) => val.span(),
            Self::Literal(val) => val.span(),
            Self::End => Span::call_site(),
        }
    }
}

impl From<TokenTree> for Token {
    fn from(tt: TokenTree) -> Self {
        match tt {
            TokenTree::Ident(val) => Self::Ident(val),
            TokenTree::Punct(val) => Self::Punct(val),
            TokenTree::Group(val) => Self::Group(val),
            TokenTree::Literal(val) => Self::Literal(val),
        }
    }
}

impl From<Option<TokenTree>> for Token {
    fn from(value: Option<TokenTree>) -> Self {
        value.map_or_else(|| Self::End, Into::into)
    }
}

impl ToTokens for Token {
    fn to_tokens(&self, to: &mut TokenStream) {
        match self {
            Self::Ident(val) => val.to_tokens(to),
            Self::Punct(val) => val.to_tokens(to),
            Self::Group(val) => val.to_tokens(to),
            Self::Literal(val) => val.to_tokens(to),
            Self::End => {}
        }
    }
}

macro_rules! impl_parse {
    ($($n:ident $name:literal),* $(,)?) => {
        $(impl Parse for $n {
            fn parse(input: &mut Input) -> Result<Self, Error> {
                tok! {
                    input.pop();
                    $n @ val => Ok(val),
                    _ => return Err(input.error(concat!($name, " expected"))),
                }
            }
        })*
    }
}

impl_parse!(
    Ident "an identifier",
    Punct "a punctuation",
    Group "a group",
    Literal "a literal",
);

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Delimiter;

    #[test]
    fn ident_parses() {
        let ts = quote!(foo);
        let mut input = Input::from(ts);
        let ident: Ident = input.parse().unwrap();
        assert_eq!(ident.to_string(), "foo");
    }

    #[test]
    fn ident_rejects_punct() {
        let ts = quote!('+');
        let mut input = Input::from(ts);
        let result: Result<Ident, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn punct_rejects_ident() {
        let ts = quote!(foo);
        let mut input = Input::from(ts);
        let result: Result<Punct, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn literal_rejects_ident() {
        let ts = quote!(foo);
        let mut input = Input::from(ts);
        let result: Result<Literal, _> = input.parse();
        assert!(result.is_err());
    }

    #[test]
    fn group_parses_braces() {
        let ts = quote!({ a, b });
        let mut input = Input::from(ts);
        let group: Group = input.parse().unwrap();
        assert_eq!(group.delimiter(), Delimiter::Brace);
    }

    #[test]
    fn from_tt_for_end() {
        let opt: Option<TokenTree> = None;
        let tok: Token = opt.into();
        assert!(matches!(tok, Token::End));
    }
}
