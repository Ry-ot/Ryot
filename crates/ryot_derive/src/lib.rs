mod manifest;

extern crate proc_macro;

use crate::manifest::Manifest;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Path};

pub(crate) fn ryot_pathfinder_path() -> Path {
    Manifest::default().get_path("ryot_pathfinder")
}

#[proc_macro_derive(Pathable)]
pub fn pathable_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let path_finder_path: Path = ryot_pathfinder_path();

    let struct_name = &input.ident;

    let expanded = quote! {
        impl #path_finder_path::pathable::Pathable for #struct_name {}
    };

    TokenStream::from(expanded)
}
