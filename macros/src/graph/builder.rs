use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, Attribute, Ident, PatType, Stmt, Token, Type, Visibility};

use crate::utils::SkipNthExt;

pub struct BuilderField {
    pub ident: Ident,
    pub generic: Ident,
    pub input: PatType,
}

pub struct GraphBuilder {
    pub attrs: Vec<Attribute>,
    pub vis: Visibility,
    pub ident: Ident,
    pub fields: Vec<BuilderField>,
    pub constness: Option<Token![const]>,
    pub asyncness: Option<Token![async]>,
    pub stmts: Vec<Stmt>,
    pub output: Type,
}

impl GraphBuilder {
    pub fn _ident(&self) -> &Ident {
        &self.ident
    }
}

impl ToTokens for GraphBuilder {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // bring all items into scope
        let attrs = &self.attrs;
        let vis = &self.vis;
        let ident = &self.ident;
        let fields = &self.fields;
        let constness = &self.constness;
        let asyncness = &self.asyncness;
        let stmts = &self.stmts;
        let output = &self.output;

        // create an ident with the span stripped
        // this is mainly to improve hover info in IDEs
        let mut unspan_ident = ident.clone();
        unspan_ident.set_span(Span::call_site());

        // build struct fields and generics
        let generics = fields.iter().map(|f| &f.generic).collect::<Vec<_>>();
        let idents = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
        let types = fields.iter().map(|f| &f.input.ty).collect::<Vec<_>>();

        // build input and wrapped output parts
        // the output has to be wrapped in a future if the function is async
        let inputs = fields.iter().map(|f| &f.input);
        let wrapped_output = match asyncness {
            Some(_) => quote! { impl ::core::future::Future<Output = #output> },
            None => quote! { #output },
        };

        // build generic structures
        let struct_generics = (!fields.is_empty()).then(|| quote! { <#(#generics = (),)*> });
        let final_impl_generics = (!fields.is_empty())
            .then(|| quote! {<#(#generics: ::choreo::GraphNode<Output = #types>,)*>});
        let final_type_generics = (!fields.is_empty()).then(|| quote! { <#(#generics,)*> });

        // create builder parts
        let builder_parts = build_builder_parts(&unspan_ident, fields);

        // create final node builder parts
        let join_chain = build_join_chain(fields);
        let tuple_idents = build_tuple_idents(fields.len());
        let joined_tuple = build_joined_tuple(tuple_idents.iter());

        // generate the builder code
        tokens.extend(quote! {
            #vis struct #ident #struct_generics {
                _private: (),
                #(
                    #idents: #generics,
                )*
            }

            impl #unspan_ident {
                pub fn new() -> Self {
                    Self {
                        _private: (),
                        #(
                            #idents: (),
                        )*
                    }
                }
            }

            #builder_parts

            impl #final_impl_generics #unspan_ident #final_type_generics {
                pub fn build_node(self) -> impl ::choreo::GraphNode<Output = #wrapped_output> {
                    #(#attrs)*
                    #constness #asyncness fn __choreo_action(#(#inputs,)*) -> #output {
                        #(#stmts)*
                    }

                    #[allow(unused_imports)]
                    use ::choreo::nodes::{JoinExt, Source, ThenExt};
                    #join_chain.then(|#joined_tuple| {

                        __choreo_action(#tuple_idents)
                    })
                }
            }
        });
    }
}

fn build_join_chain(fields: &[BuilderField]) -> TokenStream {
    match fields.len() {
        0 => quote! { Source::new(()) },
        _ => {
            let first = &fields[0].ident;
            let chain = fields.iter().skip(1).map(|f| {
                let ident = &f.ident;
                quote! { .join(self.#ident) }
            });
            quote! { self.#first #(#chain)* }
        }
    }
}

fn build_tuple_idents(count: usize) -> Punctuated<Ident, Token![,]> {
    (0..count)
        .map(|i| Ident::new(&format!("i{i}"), Span::call_site()))
        .collect::<Punctuated<_, Token![,]>>()
}

fn build_joined_tuple<'a>(idents: impl Iterator<Item = &'a Ident>) -> Option<TokenStream> {
    idents
        .map(ToTokens::to_token_stream)
        .reduce(|lhs, rhs| quote! { (#lhs, #rhs) })
}

fn build_builder_parts(ident: &Ident, fields: &[BuilderField]) -> TokenStream {
    let parts = fields.iter().enumerate().map(|(index, field)| {
        // get data for current field
        let this_ident = &field.ident;
        let this_type = &*field.input.ty;

        // collect field transfers
        let other_idents = fields
            .iter()
            .skip_nth(index)
            .map(|f| {
                let ident = &f.ident;
                quote! { #ident: self.#ident }
            })
            .collect::<Vec<_>>();

        // create generic layouts
        let generics = fields
            .iter()
            .enumerate()
            .map(|(i, f)| (i != index).then(|| &f.generic))
            .collect::<Vec<_>>();
        let impl_gen = generics
            .iter()
            .filter_map(Option::as_ref)
            .collect::<Vec<_>>();
        let type_gen = generics
            .iter()
            .map(|f| match f {
                Some(gen) => quote! { #gen },
                None => quote! { () },
            })
            .collect::<Vec<_>>();
        let out_gen = generics
            .iter()
            .map(|f| match f {
                Some(gen) => quote! { #gen },
                None => quote! { impl ::choreo::GraphNode<Output = #this_type> },
            })
            .collect::<Vec<_>>();

        // create function names
        let set_fn_ident = Ident::new(&format!("set_{}", &field.ident), Span::call_site());
        let node_fn_ident = Ident::new(&format!("{}_node", &field.ident), Span::call_site());

        // generate builder part
        quote! {
            impl<#(#impl_gen,)*> #ident<#(#type_gen,)*> {
                pub fn #set_fn_ident(self, value: #this_type) -> #ident<#(#out_gen,)*>
                {
                    use ::choreo::nodes::{Source, ThenExt};
                    #ident {
                        _private: (),
                        #(#other_idents,)*
                        #this_ident: Source::new(value),
                    }
                }

                pub fn #node_fn_ident(
                    self,
                    node: impl ::choreo::GraphNode<Output = #this_type>
                ) -> #ident<#(#out_gen,)*>
                {
                    use ::choreo::nodes::ThenExt;
                    #ident {
                        _private: (),
                        #(#other_idents,)*
                        #this_ident: node,
                    }
                }
            }
        }
    });

    quote! { #(#parts)* }
}
