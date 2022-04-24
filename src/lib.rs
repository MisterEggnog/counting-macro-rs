//!
//! The counters use i32 for the backend.
//! Only incrementing is supported.
//! ```
//! counting! {
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
//!     assert_eq!(counter_bump(count)!, 1);
//! }
//! ```

use proc_macro::TokenStream;

#[proc_macro]
pub fn counter(input: TokenStream) -> TokenStream {
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
