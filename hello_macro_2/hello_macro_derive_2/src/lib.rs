/// Derive HelloMacro2 to print struct-name and it's fields-names.
extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::{
    self,
    Data,
    Fields,
    DeriveInput,
    Ident
};

#[proc_macro_derive(HelloMacro2)]
pub fn derive_hello_macro_2(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = ast.ident;
    let fields_names: Vec<Ident> = match ast.data {
        Data::Struct(data) => {
            match data.fields {
                Fields::Named(fields) => {
                    fields.named.iter()
                        .map(|field| field.clone().ident.unwrap()).collect()
                }
                _ => unimplemented!()
            }
        }
        _ => unimplemented!()
    };

    let gen = quote!{
        impl HelloMacro2 for #name {
            fn hello_macro_2() {
                println!("Hello, Macro! Struct {}, Fields: {}",
                    stringify!(#name),
                    #( stringify!( #fields_names ) ),*
                );
            }
        }
    };

    gen.into()
}