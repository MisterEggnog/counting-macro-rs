//!
//! The counters use i32 for the backend.
//! Only incrementing is supported.
//! ```
//! # use bump_counting_macro::*;
//!  counter_create!(count);
//!
//! // Get the value of the counter & increment
//!  assert_eq!(counter_bump!(count), 0);
//!
//! // Get the value of the counter without incrementing
//! assert_eq!(counter_peek!(count), 1);
//!
//! // Change the value of the counter
//! counter_set!(count, 12);
//! assert_eq!(counter_bump!(count), 12);
//! ```
use proc_macro::TokenStream;
use std::cell::RefCell;
use std::collections::HashMap;

use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Ident, LitInt, Token};

use quote::quote;

thread_local! {
    static COUNTERS: RefCell<HashMap<String, i32>> =
        RefCell::new(Default::default());
}

#[proc_macro]
pub fn counter_bump(input: TokenStream) -> TokenStream {
    let IdentString(counter) = parse_macro_input!(input as IdentString);

    COUNTERS
        .with(|counters| {
            let mut list = counters.borrow_mut();

            let num = list[&counter];
            list.insert(counter, num + 1).unwrap();

            quote! {
                { #num }
            }
        })
        .into()
}

#[proc_macro]
pub fn counter_peek(input: TokenStream) -> TokenStream {
    let IdentString(counter) = parse_macro_input!(input as IdentString);

    COUNTERS
        .with(|counters| {
            let list = counters.borrow();

            let num = list[&counter];

            quote! {
                { #num }
            }
        })
        .into()
}

#[proc_macro]
pub fn counter_set(input: TokenStream) -> TokenStream {
    let IdentStringNum(counter, num) = parse_macro_input!(input as IdentStringNum);
    let counter = counter.to_string();

    COUNTERS.with(|counters| {
        let mut list = counters.borrow_mut();
        list.insert(counter, num);
    });

    Default::default()
}

#[proc_macro]
pub fn counter_create(input: TokenStream) -> TokenStream {
    let IdentString(counter) = parse_macro_input!(input as IdentString);

    COUNTERS.with(|counters| {
        let mut list = counters.borrow_mut();
        list.insert(counter, 0);
    });

    Default::default()
}

struct IdentString(String);

impl Parse for IdentString {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        let ident: Ident = input.parse()?;
        Ok(IdentString(ident.to_string()))
    }
}

struct IdentStringNum(String, i32);

impl Parse for IdentStringNum {
    fn parse(input: ParseStream<'_>) -> syn::parse::Result<Self> {
        let IdentString(ident) = input.parse()?;
        input.parse::<Token![,]>()?;
        let lit: LitInt = input.parse()?;
        let num = lit.base10_parse()?;

        Ok(IdentStringNum(ident, num))
    }
}
