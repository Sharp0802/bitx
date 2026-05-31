#![expect(missing_docs, reason = "will be documented in bitx crate")]

mod ast;
mod cg;
mod hir;
mod lit;
mod off;
mod prelude;

use prelude::*;
use proc_macro as pm;

fn bits_impl(ast: ast::Data) -> Result<pm::TokenStream> {
    let _hir: hir::Data = ast.try_into()?;
    //ast.to_token_stream().into()
    Ok(pm::TokenStream::new())
}

#[proc_macro]
pub fn bits(input: pm::TokenStream) -> pm::TokenStream {
    let ast = syn::parse_macro_input!(input as ast::Data);
    match bits_impl(ast) {
        Ok(output) => output,
        Err(err) => err.into_compile_error().into(),
    }
}
