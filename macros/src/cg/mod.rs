use crate::hir::Data;
use crate::prelude::*;

mod enm;
mod field;
mod layout;
mod strct;
mod value;
mod values;
mod variant;

macro_rules! to_tokens {
    (for $ty:ty; |$self:ident, $tokens:ident| { $($tt:tt)* }) => {
        impl ToTokens for $ty {
            fn to_tokens(&$self, $tokens: &mut TokenStream) {
                $($tt)*
            }
        }
    }
}

pub(crate) use to_tokens;

to_tokens!(for Data; |self, tokens| {
    match self {
        Self::Enum(enm) => enm.to_tokens(tokens),
        Self::Struct(strct) => strct.to_tokens(tokens),
    }
});
