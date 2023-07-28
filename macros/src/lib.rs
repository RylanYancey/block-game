
use std::sync::atomic::AtomicUsize;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, parse::Parser, Data};

#[proc_macro_attribute]
pub fn resource(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item = parse_macro_input!(input as DeriveInput);
    let name = item.ident.clone();

    if let Data::Struct(ref mut s) = item.data {
        if let syn::Fields::Named(ref mut fields) = s.fields {
            fields.named.push(
                syn::Field::parse_named
                    .parse2(quote! { __core_id: usize })
                    .unwrap(),
            )
        } 
    } 

    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    let counter = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Release);

    let out = quote! {
        #item

        impl #name {
            const CORE_ID: usize = #counter;
        }

        impl core::CoreId for #name {
            fn id() -> usize {
                return Self::CORE_ID
            }
        }
    }.into();

    out
}
