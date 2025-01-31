use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Expr, Ident};

pub fn build_join(mut exprs: impl Iterator<Item = Expr>) -> TokenStream {
    // ensure there is at least one input
    let Some(first) = exprs.next() else {
        return quote!({ compile_error!("`join!` must take at least one GraphNode parameter") });
    };

    // collect the rest of the expressions
    let exprs = exprs.collect::<Vec<_>>();

    // if there are no more exprs
    // just wrap the single input and return
    if exprs.is_empty() {
        return quote! { {#first} };
    }

    // create the variable names
    let vars = (0..exprs.len() + 1)
        .map(|i| Ident::new(&format!("i{i}"), Span::call_site()))
        .collect::<Vec<_>>();

    // fold the vars into a nested tuple
    let nested = vars.iter().skip(1).fold(quote! { i0 }, |lhs, rhs| {
        quote! { (#lhs, #rhs) }
    });

    // build the final join chain
    quote! {
        // nest the calculation in its own scope
        {
            // import necessary extension methods
            use ::choreo::nodes::{JoinExt, ThenExt};

            // join the nodes and flatten the tuple
            #first #(.join(#exprs))* .then(|#nested| (#(#vars,)*))
        }
    }
}
