use proc_macro::TokenStream;
use syn::{parse_macro_input, punctuated::Punctuated, Expr, Token};

mod merge;

#[proc_macro]
pub fn merge(input: TokenStream) -> TokenStream {
    // parse the input as a comma seperated list of expressions
    let exprs = parse_macro_input!(
        input with Punctuated::<Expr, Token![,]>::parse_terminated
    );

    // then build the join
    merge::build_merge(exprs.into_iter()).into()
}
