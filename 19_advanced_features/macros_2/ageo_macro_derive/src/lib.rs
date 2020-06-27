// This one allows us to manipulate Rust code
extern crate proc_macro;

use proc_macro::TokenStream;
// This one parses Rust code from string to a data structure
use syn;

// Quote does the opposite
use quote::quote;

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
  let name = &ast.ident;
  let gen = quote! {
    impl HelloMacro for #name {
      fn hello_macro() {
        println!("Hello, Macro! My name is {}!", stringify!(#name));
      }
    }
  };
  gen.into()
}

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
  // Construct a representation of Rust code as a syntax tree
  // that we can manipulate
  let ast = syn::parse(input).unwrap();

  // Build the trait implementation
  impl_hello_macro(&ast)
}
