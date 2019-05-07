extern crate proc_macro;
use proc_macro::TokenStream;

use quote::quote;
use syn::parse_macro_input;
use syn::{
    Expr,
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

#[proc_macro_attribute]
pub fn add_messages_2(attr: TokenStream, input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!( attr as Expr );
    let message =  if let Expr::Lit(lit) = expr {
        lit
    } else {
        panic!("")
    };
    let ast = parse_macro_input!( input as ItemFn );
    let fn_name = ast.ident;
    let fn_body = ast.block.stmts;
    let gen = quote!(
        fn #fn_name() {
            println!("{}", #message);
            #( #fn_body )*
        }
    );
    gen.into()
}