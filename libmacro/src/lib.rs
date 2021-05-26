use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, ItemStruct};
// use macroquad::prelude::*;
use proc_macro2::{Ident, Span};


#[proc_macro_attribute]
pub fn BaseObject(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);
    let fields_to_add = vec![];
    let name = &item_struct.ident;

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for f in fields_to_add{
            fields.named.push(
                syn::Field::parse_named
                    .parse2(f)
                    .unwrap(),
            );
        }
    }

    let name_low = Ident::new(&(name.to_string().to_lowercase())[..], Span::call_site());
    return quote! {
        #item_struct
        impl #name {
            pub fn texture(&self) -> macroquad::texture::Texture2D {
                let resources = macroquad::experimental::collections::storage::get::<crate::resources::Resources>();
                resources.#name_low
            }
        }
    }
    .into();
}
