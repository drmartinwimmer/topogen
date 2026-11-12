//! Procedural derive macros forwarding `topogen` spatial and generator traits through wrapper
//! structs annotated with `#[wrap]`.

mod bounded;
mod center;
mod euclidean;
mod generator;
mod helpers;
mod shape;

use proc_macro::TokenStream;
use syn::parse_macro_input;

/// Derives `Euclidean<T, D>` by forwarding to the struct field marked with `#[wrap]`.
#[proc_macro_derive(Euclidean, attributes(wrap))]
pub fn euclidean(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    euclidean::euclidean(input).into()
}

/// Derives `Center<T, D>` by forwarding `center()` to the struct field marked with `#[wrap]`.
#[proc_macro_derive(Center, attributes(wrap))]
pub fn center(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    center::center(input).into()
}

/// Derives `Bounded<T, D>` by forwarding `within()` and `bounding_box()` to the `#[wrap]` field.
#[proc_macro_derive(Bounded, attributes(wrap))]
pub fn bounded(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    bounded::bounded(input).into()
}

/// Derives `Shape<T, D>` by forwarding `distance()` and `signed_distance()` to the `#[wrap]` field.
#[proc_macro_derive(Shape, attributes(wrap))]
pub fn shape(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    shape::shape(input).into()
}

/// Derives `Generator<I>` by forwarding `generate()` to the struct field marked with `#[wrap]`.
#[proc_macro_derive(Generator, attributes(wrap))]
pub fn generator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);
    generator::generator(input).into()
}
