extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, Meta, MetaList, Result, Token, Expr};
use syn::parse::{Parse, ParseStream};

#[proc_macro_attribute]
pub fn auto_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let fn_name =  &input.sig.ident;

    let test_fn_name = quote::format_ident!("test_{}", fn_name);

    let expanded = quote! {
        #input

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn #test_fn_name() {
                let result = #fn_name();
                assert!(result.is_ok(), "Function {} failed", stringify!(#fn_name));
            }
        }
    };

    return expanded.into();
}

#[proc_macro_attribute]
pub fn auto_test_args(attr: TokenStream, item: TokenStream) -> TokenStream {
    use syn::{parse_macro_input, ItemFn, Expr, Token, punctuated::Punctuated};
    use quote::quote;

    // Parse the input function
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    // Parse the attributes (arguments for the test function)
    let args: Punctuated<Expr, Token![,]> = parse_macro_input!(attr with Punctuated::parse_terminated);

    // Generate a test function name
    let test_fn_name = quote::format_ident!("test_{}", fn_name);

    // Expand the input function and the generated test
    let expanded = quote! {
        #input

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn #test_fn_name() {
                let result = #fn_name(#args); // Pass the parsed arguments to the function
                assert!(result.is_ok(), "Function {} failed", stringify!(#fn_name));
            }
        }
    };

    expanded.into()
}

// A customized struct to parse the arguments
struct Args {
    cases: Vec<Expr>, // Store the test cases as expressions
}

impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut cases = Vec::new();
        while !input.is_empty() {
            let expr: Expr = input.parse()?; // Parse each test case as an expression
            cases.push(expr);

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?; // Consume the comma if it exists
            }
        }
        Ok(Args { cases })
    }
}

#[proc_macro_attribute]
pub fn auto_test_dispatchable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args);
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    // Generate test cases based on parsed arguments
    let test_fns = args.cases.iter().enumerate().map(|(i, case)| {
        let test_fn_name = quote::format_ident!("test_{}_{}", fn_name, i);

        quote! {
            #[test]
            fn #test_fn_name() {
                let result = #fn_name #case; // Apply the parsed test case arguments
                assert!(result.is_ok(), "Test case {:?} failed", #case);
            }
        }
    });

    // Expand the original function and include the generated test functions
    let expanded = quote! {
        #input

        #[cfg(test)]
        mod generated_tests {
            use super::*;

            #(#test_fns)*
        }
    };

    expanded.into()
}