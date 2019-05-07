extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;
use syn::{
    ItemFn
};

#[proc_macro_attribute]
pub fn add_messages(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!( input as ItemFn );
    let fn_name = ast.ident;
    let fn_body = ast.block.stmts;
    let gen = quote!(
        fn #fn_name() {
            println!("Hello by attr");
            #( #fn_body )*
            println!("Bye by attr");
        }
    );
    gen.into()
}
