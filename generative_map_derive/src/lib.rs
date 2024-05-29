mod euclidean;
mod helpers;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro_derive(Euclidean, attributes(wrap))]
pub fn euclidean(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    euclidean::euclidean(input).into()
}
