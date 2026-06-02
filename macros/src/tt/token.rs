use crate::prelude::*;
use crate::tt::*;

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
        match value {
            Some(tt) => tt.into(),
            None => Self::End,
        }
    }
}

impl ToTokens for Token {
    fn to_tokens(&self, to: &mut TokenStream) {
        match self {
            Self::Ident(val) => val.to_tokens(to),
            Self::Punct(val) => val.to_tokens(to),
            Self::Group(val) => val.to_tokens(to),
            Self::Literal(val) => val.to_tokens(to),
            Self::End => {},
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
