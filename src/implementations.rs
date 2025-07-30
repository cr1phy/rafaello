use proc_macro2::TokenStream;
use syn::{parse_macro_input, DeriveInput};

pub fn component_impl(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // TODO

    input.into()
}