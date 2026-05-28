#![expect(missing_docs, reason = "will be documented in bitx crate")]

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

mod data;
mod field;
mod lit;
mod off;

#[proc_macro]
pub fn bits(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as data::Struct);
    ast.to_token_stream().into()
}
