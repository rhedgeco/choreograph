use proc_macro::TokenStream;
use syn::{parse_macro_input, punctuated::Punctuated, Expr, Token};

mod join;

#[proc_macro]
pub fn join(input: TokenStream) -> TokenStream {
    // parse the input as a comma seperated list of expressions
    let exprs = parse_macro_input!(
        input with Punctuated::<Expr, Token![,]>::parse_terminated
    );

    // then build the join
    join::build_join(exprs.into_iter()).into()
}
