use convert_case::{Case, Casing};
use darling::FromMeta;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{Attribute, Ident, ImplItemFn, ItemFn, Signature, Stmt, Type, Visibility};

use crate::graph::builder::{BuilderField, GraphBuilder};

#[derive(Debug, FromMeta)]
pub struct GenArgs {
    pub name: Option<Ident>,
    pub builder: Option<Ident>,
}

pub struct GraphGenerator {
    pub graph_func: TokenStream,
    pub graph_builder: GraphBuilder,
}

impl ToTokens for GraphGenerator {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let func = &self.graph_func;
        let builder = &self.graph_builder;
        tokens.extend(quote! {
            #func
            #builder
        });
    }
}

impl GraphGenerator {
    pub fn build_item_fn(args: GenArgs, item: ItemFn) -> Self {
        Self::build_parts(None, args, item.attrs, item.vis, item.sig, item.block.stmts)
    }

    pub fn _build_impl_fn(parent: Type, args: GenArgs, item: ImplItemFn) -> Self {
        Self::build_parts(
            Some(parent),
            args,
            item.attrs,
            item.vis,
            item.sig,
            item.block.stmts,
        )
    }

    pub fn build_parts(
        _parent: Option<Type>,
        args: GenArgs,
        attrs: Vec<Attribute>,
        vis: Visibility,
        sig: Signature,
        stmts: Vec<Stmt>,
    ) -> Self {
        // generate the function ident
        let func_ident = args.name.unwrap_or_else(|| sig.ident);

        // generate the builder ident
        let builder_ident = args.builder.unwrap_or_else(|| {
            let name = format!("{func_ident}_builder").to_case(Case::Pascal);
            Ident::new(&name, func_ident.span())
        });

        // construct the graph builder
        let graph_builder = GraphBuilder {
            attrs,
            vis,
            ident: builder_ident,
            fields: sig
                .inputs
                .into_iter()
                .enumerate()
                .map(|(i, arg)| match arg {
                    syn::FnArg::Receiver(_) => unreachable!(),
                    syn::FnArg::Typed(pat_type) => BuilderField {
                        ident: Ident::new(&format!("i{i}"), Span::call_site()),
                        generic: Ident::new(&format!("I{i}"), Span::call_site()),
                        input: pat_type,
                    },
                })
                .collect(),
            constness: sig.constness,
            asyncness: sig.asyncness,
            stmts,
            output: match sig.output {
                syn::ReturnType::Default => syn::parse_quote!(()),
                syn::ReturnType::Type(_, ty) => *ty,
            },
        };

        Self {
            graph_func: TokenStream::new(),
            graph_builder,
        }
    }
}
