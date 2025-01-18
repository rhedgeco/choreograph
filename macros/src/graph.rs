use convert_case::{Case, Casing};
use darling::FromMeta;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, FnArg, Ident, ItemFn};

#[derive(Debug, FromMeta)]
pub struct GraphArgs {
    rename: Option<Ident>,
    cache_output: Option<bool>,
}

pub fn builder(args: GraphArgs, item: ItemFn) -> TokenStream {
    // get simple data fields out of item
    let vis = item.vis;
    let attrs = item.attrs;
    let ident = item.sig.ident;
    let inputs = item.sig.inputs;
    let task_output = item.sig.output;
    let statements = item.block.stmts;
    let asyncness = item.sig.asyncness;
    let constness = item.sig.constness;

    // if builder rename is available, use that is builder ident
    // otherwise convert the function name into an ident
    let spanned_builder = args.rename.unwrap_or_else(|| {
        let pascal_case = format!("{ident}_builder").to_case(Case::Pascal);
        Ident::new(&pascal_case, ident.span())
    });

    // create an unspanned builder ident so that inspection only shows struct def
    let mut builder = spanned_builder.clone();
    builder.set_span(Span::call_site());

    // collect generics data from function signature
    let (genimpl, gentype, genwhere) = item.sig.generics.split_for_impl();

    // build the task inputs
    let task_inputs = inputs
        .iter()
        .map(|input| {
            if asyncness.is_none() {
                input.clone()
            } else {
                input.clone()
            }
        })
        .collect::<Punctuated<_, Comma>>();

    quote! {
        #vis fn #ident() -> #builder {
            todo!()
        }

        #vis struct #spanned_builder {}

        impl #builder {
            pub fn build() {
                #(#attrs)*
                #asyncness #constness fn task(#task_inputs) #task_output {
                    #(#statements)*
                }
            }
        }
    }
    .into()
}
