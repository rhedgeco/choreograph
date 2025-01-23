use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, FnArg, Ident, ItemFn};

#[derive(Debug, FromMeta)]
pub struct GraphArgs {
    rename: Option<Ident>,
    #[darling(default)]
    async_inputs: bool,
}

pub fn builder(args: GraphArgs, item: ItemFn) -> TokenStream {
    // get simple data fields out of item
    let attrs = item.attrs;
    let vis = item.vis;
    let constness = item.sig.constness;
    let asyncness = item.sig.asyncness;
    let ident = item.sig.ident;
    let block = item.block;
    let inputs = item.sig.inputs;
    let output = item.sig.output;

    // create a wrapped version of the inputs with futures if necessary
    let wrapped_inputs = match args.async_inputs || asyncness.is_some() {
        false => inputs.clone(),
        true => inputs
            .iter()
            .map(|arg| match arg {
                FnArg::Receiver(_) => arg.clone(),
                FnArg::Typed(pat_type) => {
                    let mut pat_type = pat_type.clone();
                    let mut ty = pat_type.ty;
                    ty = syn::parse_quote!(impl ::std::future::Future<Output = #ty>);
                    pat_type.ty = ty;
                    FnArg::Typed(pat_type)
                }
            })
            .collect(),
    };

    // create the function inputs by removing all `self`
    let func_inputs = wrapped_inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(pat_type) => Some(pat_type.clone()),
            FnArg::Receiver(_) => None,
        })
        .collect::<Punctuated<_, Comma>>();

    // create the ident for the builder function
    let builder_ident = match args.rename {
        Some(renamed_ident) => renamed_ident,
        None => ident.clone(),
    };

    // convert the inputs into graph nodes
    let builder_inputs = func_inputs
        .iter()
        .map(|pat_type| {
            let mut pat_type = pat_type.clone();
            let mut ty = pat_type.ty;
            // wrap the type in a graph node
            ty = syn::parse_quote!(impl ::choreo::GraphNode<Output = #ty>);
            pat_type.ty = ty; // put the type back into the pattern
            pat_type
        })
        .collect::<Punctuated<_, Comma>>();

    // create the output type of the builder function
    let mut builder_output = match &output {
        syn::ReturnType::Default => syn::parse_quote!(()),
        syn::ReturnType::Type(_, ty) => *ty.clone(),
    };
    // if the function is async, ensure it is wrapped in the builder output
    if asyncness.is_some() {
        builder_output = syn::parse_quote!(impl ::std::future::Future<Output = #builder_output>);
    }

    // create the node builder
    let node_builder = match func_inputs.len() {
        0 => quote!(Action::new(|_| #ident())),
        1 => {
            let input_pat = (&*func_inputs[0].pat).clone();
            quote!(#input_pat.then(|i| #ident(i)))
        }
        _ => {
            // create the node join chain
            let mut pat_iter = func_inputs.iter().map(|a| &*a.pat);
            let first_pat = pat_iter.next().unwrap();
            let second_pat = pat_iter.next().unwrap();
            let mut join_chain = quote! { #first_pat.join(#second_pat) };
            for pat in pat_iter {
                join_chain = quote! { #join_chain.join(#pat) }
            }

            // create the resulting tuple from the join chain
            let mut join_tuple = quote!((i0, i1));
            for index in 2..func_inputs.len() {
                let ident: Ident = Ident::new(&format!("i{index}"), Span::call_site());
                join_tuple = quote! {(#join_tuple, #ident)};
            }

            // create the call input for the function
            let call_input = (0..func_inputs.len())
                .map(|index| Ident::new(&format!("i{index}"), Span::call_site()))
                .collect::<Punctuated<_, Comma>>();

            // create the final graph node builder
            quote!(#join_chain.then(|#join_tuple| #ident(#call_input)))
        }
    };

    // collect generics data from function signature
    let (genimpl, _, genwhere) = item.sig.generics.split_for_impl();

    quote! {
        #vis fn #builder_ident #genimpl (#builder_inputs) -> impl ::choreo::GraphNode<Output = #builder_output> #genwhere {
            #(#attrs)*
            #constness #asyncness fn #ident #genimpl (#func_inputs) #output #genwhere #block

            #[allow(unused_imports)]
            use ::choreo::nodes::{Action, JoinExt, ThenExt};
            #node_builder
        }
    }
    .into()
}
