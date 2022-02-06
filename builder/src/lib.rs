use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, Data, DeriveInput, Fields, __private::quote::quote_spanned, spanned::Spanned,
};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let children = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|f| {
                    let name = &f.ident;
                    quote_spanned! {f.span()=> Option<&self.#name>}
                });
                quote! {
                    0 #(+ #recurse)*
                }
            }
            _ => unimplemented!(),
        },
        _ => unimplemented!(),
    };
    let token = quote! {
        impl #name{
            pub fn builder(){}
        }
        pub struct CommandBuilder{
            #children
        }
    };
    TokenStream::from(token)
}
