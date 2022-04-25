//!
//! The counters use i32 for the backend.
//! Only incrementing is supported.
//! ```
//! /*counting! {
//!     counter_create!(count);
//!
//!     // Get the value of the counter & increment
//!     assert_eq!(counter_bump!(count), 0);
//!
//!     // Get the value of the counter without incrementing
//!     assert_eq!(counter_peek!(count), 1);
//!
//!     // Change the value of the counter
//!     counter_set!(count, 1);
//!     assert_eq!(counter_bump!(count), 1);
//! }*/
//! ```
use lazy_static::lazy_static;
use proc_macro::TokenStream;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use syn::parse_macro_input;
use syn::Ident;

use quote::quote;

lazy_static! {
    static ref COUNTERS: Arc<Mutex<HashMap<String, i32>>> =
        Arc::new(Mutex::new(Default::default()));
}

#[proc_macro]
pub fn counting(_input: TokenStream) -> TokenStream {
    Default::default()
}

#[proc_macro]
pub fn counter_bump(input: TokenStream) -> TokenStream {
    let counter = parse_macro_input!(input as Ident);
    let counter = format!("{}", counter);

    let counter_list = COUNTERS.clone();
    let mut list = counter_list.lock().unwrap();

    let num = list[&counter];
    list.insert(counter, num + 1).unwrap();

    quote! {
        { #num }
    }
    .into()
}

#[proc_macro]
pub fn counter_peek(input: TokenStream) -> TokenStream {
    let counter = parse_macro_input!(input as Ident);
    let counter = format!("{}", counter);

    let counter_list = COUNTERS.clone();
    let list = counter_list.lock().unwrap();

    let num = list[&counter];

    quote! {
        { #num }
    }
    .into()
}

#[proc_macro]
pub fn counter_set(_input: TokenStream) -> TokenStream {
    Default::default()
}

#[proc_macro]
pub fn counter_create(input: TokenStream) -> TokenStream {
    let counter = parse_macro_input!(input as Ident);
    let counter = format!("{}", counter);

    let counter_list = COUNTERS.clone();
    let mut list = counter_list.lock().unwrap();
    list.insert(counter, 0);

    Default::default()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
