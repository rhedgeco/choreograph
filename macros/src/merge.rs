use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Expr, Token};

pub fn build(input: TokenStream) -> TokenStream {
    // parse the input as a comma seperated list of expressions
    let exprs = parse_macro_input!(
        input with Punctuated::<Expr, Token![,]>::parse_terminated
    );

    // ensure there is at least one input
    if exprs.is_empty() {
        return quote!({ compile_error!("`merge!` must take at least one GraphNode parameter") })
            .into();
    }

    // make exprs into an iterator
    let exprs = exprs.into_iter();

    // build the merge action
    quote! {
        {
            // import necessary extension methods
            use ::choreo::{GraphNode, nodes::Source};

            // merge the nodes in an action that executes them
            Source::new(|| (#(#exprs.execute(),)*))
        }
    }
    .into()
}
