extern crate proc_macro;

use syn;

use quote::quote;

use crate::proc_macro::TokenStream;

#[proc_macro_derive(Test)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;

    // Build the trait implementation
    return TokenStream::from(quote! {
        impl Test for #name{
            fn test_fn(&self) -> &'static str{
                return "Cool";
            }
        }
    });
}