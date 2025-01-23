// Generate extreme values for the underlying type of a given type aliases
#[macro_export]
macro_rules! generate_extreme_values {
    () => {
        vec![Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new(&u128::MIN.to_string(), proc_macro2::Span::call_site())),
        }), Expr::Lit(ExprLit {
            attrs: vec![],
            lit: Lit::Int(LitInt::new(&u128::MAX.to_string(), proc_macro2::Span::call_site())),
        })]
    };
}
