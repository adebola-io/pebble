use proc_macro::TokenStream;
// use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Location)]
pub fn derive_location(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    TokenStream::from(quote! {
        impl Location for #name<'_> {
            fn get_range(&self) -> [[u64; 2]; 2] {
                self.span
            }
        }
    })
}
