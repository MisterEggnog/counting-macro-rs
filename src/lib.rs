//! Stateful macro for compile time counting.
//!
//! The counters use i32 for the backend.
//! Only incrementing is supported.
//! ```
//! # use counting_macros::*;
//! counter_create!(count);
//!
//! // Get the value of the counter & increment
//! assert_eq!(counter_incr!(count), 0);
//!
//! // Get the value of the counter without incrementing
//! assert_eq!(counter_peek!(count), 1);
//!
//! // Increment without getting value
//! counter_next!(count);
//! assert_eq!(counter_peek!(count), 2);
//!
//! // Change the value of the counter
//! counter_set!(count, 12);
//! assert_eq!(counter_incr!(count), 12);
//! ```
//!
//! # Warning
//! I'm not certain about the stability or safety of this, so I would not
//! recomend this for use in serious projects.
use proc_macro::TokenStream;
use std::cell::RefCell;
use std::collections::HashMap;

use syn::parse::{Parse, ParseStream};
use syn::parse_macro_input;
use syn::{Ident, LitInt, Token};

use quote::quote;

// I have been unable to find any explanations of the parallization model of rustc,
// I find it unlikely that multiple threads will be modifying a source file at the
// same time.
// Thus we are using thread_local.
//
// This library only works if rustc compiles each file in it's own thread &
// expands macros as they linearly appear in the file.
// If this is not the case then this entire concept should be scrapped.
thread_local! {
    static COUNTERS: RefCell<HashMap<String, i32>> =
        RefCell::new(Default::default());
}

/// Get counter value then increment it.
///
/// ```
/// # use counting_macros::*;
/// # counter_create!(count);
/// assert_eq!(counter_incr!(count), 0);
/// assert_eq!(counter_incr!(count), 1);
/// ```
#[proc_macro]
pub fn counter_incr(input: TokenStream) -> TokenStream {
    let IdentString(counter) = parse_macro_input!(input as IdentString);

    COUNTERS.with(|counters| {
        let mut list = counters.borrow_mut();

        let num = list[&counter];
        list.insert(counter, num + 1).unwrap();

        quote! {
            { #num }
        }
        .into()
    })
}

/// See current count without incrementing it.
///
/// ```
/// # use counting_macros::*;
/// # counter_create!(count);
/// assert_eq!(counter_peek!(count), 0);
/// assert_eq!(counter_incr!(count), 0);
/// ```
#[proc_macro]
pub fn counter_peek(input: TokenStream) -> TokenStream {
    let IdentString(counter) = parse_macro_input!(input as IdentString);

    COUNTERS.with(|counters| {
        let list = counters.borrow();

        let num = list[&counter];

        quote! {
            { #num }
        }
        .into()
    })
}

/// Change value of counter.
///
/// Counter takes an i32 integer, it can be negative.
/// ```
/// # use counting_macros::*;
/// # counter_create!(count);
/// counter_set!(count, -4);
/// assert_eq!(counter_incr!(count), -4);
/// ```
#[proc_macro]
pub fn counter_set(input: TokenStream) -> TokenStream {
    let IdentStringNum(counter, num) = parse_macro_input!(input as IdentStringNum);

    COUNTERS.with(|counters| {
        let mut list = counters.borrow_mut();
        list.insert(counter, num);
    });

    Default::default()
}

/// Create new counter.
///
/// This counter begins with a value of 0, if you want to change it use
/// [counter_set!].
/// ```
/// # use counting_macros::*;
/// counter_create!(count);
/// assert_eq!(counter_incr!(count), 0);
/// ```
#[proc_macro]
pub fn counter_create(input: TokenStream) -> TokenStream {
    let IdentString(counter) = parse_macro_input!(input as IdentString);

    COUNTERS.with(|counters| {
        let mut list = counters.borrow_mut();
        list.insert(counter, 0);
    });

    Default::default()
}

/// Increment counter.
///
/// ```
/// # use counting_macros::*;
/// # counter_create!(count);
/// assert_eq!(counter_incr!(count), 0);
/// counter_next!(count);
/// assert_eq!(counter_incr!(count), 2);
/// ```
#[proc_macro]
pub fn counter_next(input: TokenStream) -> TokenStream {
    counter_incr(input);

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

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
#[proc_macro]
pub fn ReadmeDoctests(_: TokenStream) -> TokenStream {}
