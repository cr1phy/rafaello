use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DataStruct, DeriveInput, Ident};

use crate::RenderInput;

pub fn component_impl(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;
    let data = &input.data;

    match data {
        Data::Struct(DataStruct { fields, .. }) => {
            let state = Ident::new(&format!("__State{ident}"), ident.span());
            let component = Ident::new(&format!("__Component{ident}"), ident.span());

            quote! {
                struct #state #fields
                struct #component {}
            }
        }
        _ => unimplemented!(),
    }
}

pub fn render_impl(input: RenderInput) -> TokenStream {
    quote! {#input}.into()
}
