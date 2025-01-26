use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::ItemImpl;

#[derive(Debug, FromMeta)]
pub struct ImplArgs {
    // nothing for now
}

pub struct GraphImpl {}

impl ToTokens for GraphImpl {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            // todo
        });
    }
}

impl GraphImpl {
    pub fn build(_args: ImplArgs, item: ItemImpl) -> Self {
        todo!()
    }
}
