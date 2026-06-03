#![expect(missing_docs, reason = "will be documented in bitx crate")]

mod ast;
mod cg;
mod hir;
mod prelude;
mod tt;

use proc_macro::TokenStream;
use quote::ToTokens;

#[proc_macro]
pub fn bits(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    let mut input: tt::Input = input.into();

    let ast: ast::Data = match input.parse() {
        Ok(ast) => ast,
        Err(err) => return err.to_token_stream().into(),
    };

    let hir: hir::Data = match ast.try_into() {
        Ok(hir) => hir,
        Err(err) => return err.to_token_stream().into(),
    };

    hir.to_token_stream().into()
}
