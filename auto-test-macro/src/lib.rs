extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::LitBool;
use syn::{parse_macro_input, ItemFn, Result, Token, Expr, punctuated::Punctuated, Lit, ExprLit};
use syn::parse::{Parse, ParseStream};

// A convinient mental model for procedural macros:
// Frontend: Input codes -> syn crate -> intermediate representation (This can be AST or customized types)
// Backend: Intermediate representation -> quote crate -> Output codes

#[proc_macro_attribute]
pub fn auto_test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // syn::parse_macro_input -> parse the input TokenStream of a macro
    let input = parse_macro_input!(item as ItemFn);
    // ident -> a word of Rust code, which may be a keyword or legal variable name
    let fn_name =  &input.sig.ident;

    let test_fn_name = quote::format_ident!("test_{}", fn_name);

    // Performs variable interpolation
    // The output type is proc_macro2::TokenStream, which needs to be converted to proc_macro::TokenStream use .into()
    let expanded = quote! {
        // Keep the original function definition as is and include it in the generated code
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
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;

    // Parsing macro attributes into a structured format
    // A punctuated sequence of syntax tree nodes of type Expr, separated by commas
    // Example: #[auto_test_args(1, "hello", true)] -> [1, "hello", true]
    let args: Punctuated<Expr, Token![,]> = parse_macro_input!(attr with Punctuated::parse_terminated);

    // Generate a function name for the unit test
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
    expected_results: Expr, // True for success, false for failure
}

// Customized parsing logic for syn crate to convert the input TokenStream into a structured format
impl Parse for Args {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut cases = Vec::new();
        let mut expected_results = Expr::Lit(ExprLit {
            attrs: vec![], // Empty attributes
            lit: Lit::Bool(LitBool {
                value: true,
                span: proc_macro2::Span::call_site(), // Default span
            }),
        });
        // Parse test cases
        while !input.is_empty() {
            let expr: Expr = input.parse()?;
            cases.push(expr);
            
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }

            if input.peek(Token![&]) {
                input.parse::<Token![&]>()?;
                expected_results = input.parse::<Expr>()?;
                break;
            }
        }

        Ok(Args { cases, expected_results })
    }
}

#[proc_macro_attribute]
pub fn auto_test_dispatchable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as Args);
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let expected_results = args.expected_results;
    let mut test_name_suffix = "failure";
    // Generate test cases based on parsed arguments
    let test_fns = args.cases.iter().enumerate().map(|(i, case)| {
        let test_fn_name = quote::format_ident!("test_{}_{}", fn_name, i);
        // Strip the last element from the test case tuple and make the rest as type Expr

        quote! {
            #[test]
            fn #test_fn_name() {
                let result = #fn_name #case; // Apply the parsed test case arguments
                if #expected_results {
                    assert!(result.is_ok(), "Test case {:?} failed", #case);
                } else {
                    assert!(result.is_err(), "Test case {:?} should fail", #case);
                }
            }
        }
    });

    // Determine the test name suffix based on the expected results
    if let Some(boolean_value) = match expected_results {
        Expr::Lit(ExprLit { lit: Lit::Bool(LitBool { value, .. }), .. }) => Some(value),
        _ => None,
    } {
        if boolean_value {
            test_name_suffix = "success";
        }
    }
    
    // Dynamically generate the test module name to avoid name collision when multiple macros are used in the same file
    let test_name = quote::format_ident!("{}_tests_{}", fn_name, test_name_suffix);

    // Expand the original function and include the generated test functions
    let expanded = quote! {
        #input

        #[cfg(test)]
        mod #test_name {
            use super::*;

            // Interpolate with 0 or more test functions
            #(#test_fns)*
        }
    };

    expanded.into()
}