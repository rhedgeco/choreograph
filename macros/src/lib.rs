use darling::{ast::NestedMeta, Error, FromMeta};
use graph::{
    r#impl::{GraphImpl, ImplArgs},
    GenArgs, GraphGenerator,
};
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;

mod graph;
mod utils;

#[proc_macro_attribute]
pub fn graph_builder(args: TokenStream, item: TokenStream) -> TokenStream {
    // parse the arguments into list of NestedMeta
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };

    // parse the item input and match on the item kind
    match parse_macro_input!(item as syn::Item) {
        // if its a free function just build a single graph
        syn::Item::Fn(item_fn) => {
            // parse the attribute args into the function args
            let args = match GenArgs::from_list(&attr_args) {
                Ok(v) => v,
                Err(e) => {
                    return TokenStream::from(e.write_errors());
                }
            };

            // then build the graph func
            GraphGenerator::build_item_fn(args, item_fn).to_token_stream()
        }
        syn::Item::Impl(item_impl) => {
            // parse the attribute args into the impl args
            let args = match ImplArgs::from_list(&attr_args) {
                Ok(v) => v,
                Err(e) => {
                    return TokenStream::from(e.write_errors());
                }
            };

            // then build the graph impl
            GraphImpl::build(args, item_impl).to_token_stream()
        }
        item => quote! {
            compile_error!("#[choreo::graph] can only be applied to `impl` and `fn` items");
            #item
        },
    }
    .into()
}
