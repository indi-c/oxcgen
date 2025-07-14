extern crate proc_macro;

use paste::paste;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident};

#[proc_macro]
pub fn make_asset(input: TokenStream) -> TokenStream {
    let ident = parse_macro_input!(input as Ident);
    paste! {
        let folder_path = format!("templates/{}/", ident);
    }

    let expanded = quote! {
        paste::paste! {
            #[derive(rust_embed::RustEmbed)]
            #[folder = #folder_path]
            #[prefix = concat!(stringify!(#ident), "/")]
            pub struct [< #ident Assets>];
        }
    };
    TokenStream::from(expanded)
}