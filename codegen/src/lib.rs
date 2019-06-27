extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemFn};

/// Run a future as a test, this expands to calling the `async fn` via
/// `futures::executor::block_on`. Based off the `tokio-async-await-test`
/// crate.
#[proc_macro_attribute]
pub fn async_test(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemFn);

    let test_case_name = input.ident.clone();

    let expanded = quote! {
        #[test]
        fn #test_case_name () {
            use futures::{executor::block_on, future::{FutureExt, TryFutureExt}};

            #input

            block_on(#test_case_name()).unwrap();
        }
    };

    TokenStream::from(expanded)
}

/// Generate #[async_test]s for each supported database backend.
///
/// Expects an `async fn` taking one argument: a `Box<dyn Db>`
#[proc_macro_attribute]
pub fn db_test(metadata: TokenStream, input: TokenStream) -> TokenStream {
    _db_test(metadata, input).unwrap()
}

fn _db_test(_metadata: TokenStream, input: TokenStream) -> Result<TokenStream, syn::Error> {
    let function: syn::ItemFn = syn::parse(input).expect("Parse input as function");
    let name = function.ident.clone();
    let mysql_name = syn::Ident::new(&format!("mysql_{}", &function.ident), function.ident.span());
    let spanner_name = syn::Ident::new(
        &format!("spanner_{}", &function.ident),
        function.ident.span(),
    );

    let f = quote! {
        #[async_test]
        async fn #mysql_name() -> Result<()> {
            #name(mysql_db().await?).await
        }

        /*
        #[test]
        fn #spanner_name() -> Result<()> {
            #name(spanner_db().await?).await
        }
        */

        #function
    };
    Ok(f.into())
}
