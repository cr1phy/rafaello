extern crate proc_macro;

use darling::ast::NestedMeta;
use proc_macro::TokenStream;
use syn::{braced, bracketed, parse::{Parse, ParseStream}, parse_macro_input, Attribute, DeriveInput, Ident, ItemFn, ItemImpl, LitStr, Meta, Token};
use quote::{format_ident, quote};

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(input);
    let self_ty = &ast.ident;
    let mut impl_block: Option<ItemImpl> = None;

    // ищем impl самого типа и метод #[render]
    for item in ast.attrs.iter().filter_map(attr_to_item_impl) {
        impl_block = Some(item.clone());
    }
    let impl_block = impl_block.expect("impl block required");

    // собираем (Variant, method) из #[handler]
    let handlers = impl_block
        .items
        .iter()
        .filter_map(|it| match it {
            syn::ImplItem::Fn(f) => {
                f.attrs.iter().find(|a| a.path().is_ident("rafaello_internal")).map(|a| {
                    let variant: syn::Ident = a.parse_args().unwrap();
                    let name     = &f.sig.ident;
                    quote!( #variant => self.#name(), )
                })
            }
            _ => None,
        }).clone();

    // имя enum-а
    let msg_enum = format_ident!("{}Msg", self_ty);

    // ищем render-метод
    let render_fn = impl_block.items.iter().find_map(|it| match it {
        syn::ImplItem::Fn(f) if f.attrs.iter().any(|a| a.path().is_ident("render")) => Some(f),
        _ => None,
    }).expect("Need #[render] fn");

    let render_name = &render_fn.sig.ident;

    let expanded = quote! {
        enum #msg_enum { #( #handlers )* }

        impl rafaello_types::Component for #self_ty {
            type Msg = #msg_enum;
            fn update(&mut self, msg: Self::Msg) {
                match msg {
                    #( #handlers )*
                }
            }
            fn view(&self, f: &mut ratatui::Frame<'_>, area: ratatui::layout::Rect) {
                self.#render_name()(f, area)
            }
        }
    };
    TokenStream::from(expanded)
}

fn attr_to_item_impl(attr: &Attribute) -> Option<ItemImpl> {
    if attr.path().is_ident("thriller") { None } else { None }
}

#[proc_macro_attribute]
pub fn render(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut fun = parse_macro_input!(item as ItemFn);

    // пользователь объявил fn render(&self) -> draw!{…}.  
    // Мы меняем сигнатуру, чтобы она возвращала замыкание, принимающее Frame+Rect
    fun.sig.output = syn::parse_quote!(-> impl Fn(&mut ratatui::Frame<'_>, ratatui::layout::Rect) + '_);

    // тело функции остаётся нетронутым – внутри будет draw! { … }
    TokenStream::from(quote!(#fun))
}

struct HandlerInput {
    name: Ident,
}

impl Parse for HandlerInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        Ok(HandlerInput { name })
    }
}

#[proc_macro_attribute]
pub fn handler(args: TokenStream, item: TokenStream) -> TokenStream {
    // args – либо пусто, либо (Plus) / ("Plus")
    let variant = parse_macro_input!(args as Attribute)
        .and_then(|meta| match meta {
            NestedMeta::Meta(Meta::Path(p)) => p.get_ident().cloned(),
            _ => None,
        })
        .expect("#[handler] requires variant name");

    // прикрепляем к функции скрытый атрибут, чтобы derive смог найти
    let mut fun = parse_macro_input!(item as ItemFn);
    fun.attrs.push(syn::parse_quote!(#[rafaello_internal(msg = #variant)]));

    TokenStream::from(quote!(#fun))
}

#[proc_macro]
pub fn draw(input: TokenStream) -> TokenStream {
    // Парсим только самый верхний `block [...] { ... }`
    let DrawRoot { title, content } = parse_macro_input!(input as DrawRoot);

    let maybe_title = title.map(|s| quote!( .title(#s) ));

    // Внутри пока допускаем минимум: p{} и кнопки
    let inner = content.value();

    let tokens = quote! {
        move |f: &mut ratatui::Frame<'_>, area: ratatui::layout::Rect| {
            use ratatui::widgets::{Block, Borders, Paragraph};
            let block = Block::default()
                #maybe_title
                .borders(Borders::ALL);
            f.render_widget(block, area);

            // просто рисуем один Paragraph под рамкой
            let inner_area = ratatui::layout::Rect { x: area.x+1, y: area.y+1, width: area.width-2, height: area.height-2 };
            let p = Paragraph::new(#inner);
            f.render_widget(p, inner_area);
        }
    };
    TokenStream::from(tokens)
}

struct DrawRoot {
    title: Option<String>,
    content: LitStr,
}
impl syn::parse::Parse for DrawRoot {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        // block
        let _block_kw: Ident = input.parse()?;
        // [ attrs ]
        let attrs;
        bracketed!(attrs in input);
        let title: Option<String> = if !attrs.is_empty() {
            let _title_kw: Ident = attrs.parse()?;
            let _eq: Token![=] = attrs.parse()?;
            let lit: LitStr = attrs.parse()?;
            Some(lit.value())
        } else { None };
        // { content }
        let braced;
        braced!(braced in input);
        let content: LitStr = braced.parse()?;
        Ok(Self { title, content })
    }
}
