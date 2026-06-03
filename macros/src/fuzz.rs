#![allow(missing_docs)]
#![no_main]

mod ast;
mod cg;
mod hir;
mod prelude;
mod tt;

use libfuzzer_sys::fuzz_target;
use proc_macro2::TokenStream;
use quote::ToTokens;

fuzz_target!(|data: &str| {
    if let Ok(tokens) = data.parse::<TokenStream>() {
        let mut input: tt::Input = tokens.into();

        let Ok(ast) = input.parse::<ast::Data>() else {
            return;
        };

        let Ok(hir) = TryInto::<hir::Data>::try_into(ast) else {
            return;
        };

        _ = hir.to_token_stream();
    }
});
