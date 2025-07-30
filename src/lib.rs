use proc_macro::TokenStream;

mod implementations;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    implementations::component_impl(input).into()
}