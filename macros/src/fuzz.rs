#![allow(missing_docs)]
#![no_main]

mod data;
mod lit;
mod field;
mod off;
mod variant;

use data::Data;
use libfuzzer_sys::fuzz_target;
use proc_macro2::TokenStream;
use syn::parse2;

fuzz_target!(|data: &str| {
    if let Ok(tokens) = data.parse::<TokenStream>() {
        let _ = parse2::<Data>(tokens);
    }
});

