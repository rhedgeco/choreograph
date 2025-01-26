use proc_macro2::Span;
use syn::{visit_mut, FnArg, PatType, Path, Token, Type};

pub struct ReplaceSelf<'a> {
    explicit: &'a Type,
    token: &'a str,
}

impl<'a> ReplaceSelf<'a> {
    pub fn with(token: &'a str, explicit: &'a Type) -> Self {
        Self { explicit, token }
    }
}

impl visit_mut::VisitMut for ReplaceSelf<'_> {
    fn visit_fn_arg_mut(&mut self, arg: &mut FnArg) {
        // replace all self arguments with explicit types
        if let FnArg::Receiver(rec) = arg {
            let token = &self.token;
            *arg = FnArg::Typed(PatType {
                attrs: rec.attrs.clone(),
                pat: syn::parse_quote_spanned! {
                    rec.self_token.span => #token
                },
                colon_token: Token![:](Span::call_site()),
                ty: rec.ty.clone(),
            });
        }

        // then perform normal visit
        visit_mut::visit_fn_arg_mut(self, arg);
    }

    fn visit_path_mut(&mut self, path: &mut Path) {
        // replace all self paths with a underscored one
        if path.segments.len() == 1 {
            let segment = &mut path.segments[0];
            match segment.ident.to_string().as_str() {
                "self" => {
                    let token = &self.token;
                    segment.ident = syn::parse_quote_spanned! {
                        segment.ident.span() => #token
                    }
                }
                "Self" => {
                    let explicit = &self.explicit;
                    segment.ident = syn::parse_quote_spanned! {
                        segment.ident.span() => #explicit
                    }
                }
                _ => {} // do nothing
            }
        }

        // then perform normal visit
        visit_mut::visit_path_mut(self, path);
    }
}
