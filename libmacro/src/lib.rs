use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, ItemStruct, ItemImpl, ImplItem};
// use macroquad::prelude::*;
use proc_macro2::{Ident, Span};


#[proc_macro_attribute]
pub fn BaseObject(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);
    let fields_to_add = vec![quote!{texture: macroquad::texture::Texture2D }];
    let name = &item_struct.ident;
    // if
    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        for f in fields_to_add{
            fields.named.push(
                syn::Field::parse_named
                    .parse2(f)
                    .unwrap(),
            );
        }
    }

    let name_res = Ident::new(&format!("resources.{}",  name.to_string().to_lowercase())[..], Span::call_site());
    return quote! {
        #item_struct
        impl #name {
            pub fn get_texture(&self) -> macroquad::texture::Texture2D {
                let resources = macroquad::experimental::collections::storage::get::<crate::resources::Resources>();
                #name_res
            }
        }
    }
    .into();
}

#[proc_macro_attribute]
pub fn DefaultDraw(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut ii= parse_macro_input!(input as ItemImpl);
    let a = &ii.attrs;
    let u = &ii.unsafety;
    let s = &ii.self_ty;
    let (ba, pa, fo) = ii.trait_.unwrap();
    let g = &ii.generics;
    let i = &ii.impl_token;
    let it = &ii.items;
    let d = &ii.defaultness;
    return quote! {
       #(#a)*
        #d #u #i #g #ba #pa #fo #s {
            #(#it)*
            
            fn draw(&self, x: f32, y: f32){
                macroquad::texture::draw_texture(
                    self.texture,
                    x - self.texture.width()  / 2.,
                    y - self.texture.height() / 2.,
                    macroquad::color::BLACK
                )
            }
        }
    }
    .into()
    // TokenStream::new()
}
/*  
    fn draw(&self, x: f32, y: f32){
        draw_texture(
            self.texture(),
            x - self.texture().width()  / 2.,
            y - self.texture().height() / 2.,
            BLACK
        )
    }

  */
