//#![feature(async_await, await_macro, futures_api)]

extern crate proc_macro;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

/// Run a future as a test, this expands to calling the `async fn` via `Runtime::block_on`.
#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let test_case_name = input.ident.clone();

    let expanded = quote! {
        #[test]
        fn #test_case_name () {
            use futures::{executor::block_on, future::{FutureExt, TryFutureExt}};
            //use tokio::runtime::Runtime;
            //use futures::future::{FutureExt, TryFutureExt};

            //let mut rt = Runtime::new().unwrap();

            #input

            //rt.block_on(#test_case_name().unit_error().boxed().compat()).unwrap();
            block_on(#test_case_name()).unwrap();
        }
    };

    TokenStream::from(expanded)
}
