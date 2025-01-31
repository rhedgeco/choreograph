use proc_macro::TokenStream;

mod merge;

#[proc_macro]
pub fn merge(input: TokenStream) -> TokenStream {
    merge::build(input)
}
