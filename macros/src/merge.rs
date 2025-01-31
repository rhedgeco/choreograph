use proc_macro2::TokenStream;
use quote::quote;
use syn::Expr;

pub fn build_merge(mut exprs: impl Iterator<Item = Expr>) -> TokenStream {
    // ensure there is at least one input
    let Some(first) = exprs.next() else {
        return quote!({ compile_error!("`merge!` must take at least one GraphNode parameter") });
    };

    // if there are no more exprs
    // just wrap the single input and return
    let Some(second) = exprs.next() else {
        return quote! { {#first} };
    };

    // otherwise build the merge action
    quote! {
        {
            // import necessary extension methods
            use ::choreo::{GraphNode, nodes::Source};

            // merge the nodes in an action that executes them
            Source::new(|| (#first.execute(), #second.execute() #(, #exprs.execute())*))
        }
    }
}
