extern crate proc_macro;
use std::vec;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, parse_macro_input, DeriveInput};

mod implementations;

#[proc_macro_derive(Component)]
pub fn derive_component(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    implementations::component_impl(input).into()
}

struct RenderInput {
    components: Vec<&'static str>,
}

impl Parse for RenderInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(RenderInput { components: vec!["Hello"] })
    }
}

impl ToTokens for RenderInput {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for i in self.components.iter() {
            i.to_tokens(tokens);
        }
    }
}

#[proc_macro]
pub fn render(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as RenderInput);
    implementations::render_impl(input).into()
}

#[proc_macro_attribute]
pub fn handler(args: TokenStream, item: TokenStream) -> TokenStream {
    item
}
