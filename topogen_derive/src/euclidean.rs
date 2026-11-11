use crate::helpers::{create_forward_generics, expand_generics, find_wrapped};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

pub fn euclidean(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = find_wrapped(&input).expect("Failed to find a field with the #[wrap] attribute");
    let wrapped_ty = &wrapped.ty;
    let forward_generics = create_forward_generics(&input);
    let generics = expand_generics(
        &input,
        &[
            parse_quote!(EuclideanCoordinate: topogen::traits::Coordinate),
            parse_quote!(const EuclideanDim: usize),
        ],
        &[parse_quote!(#wrapped_ty: Euclidean<EuclideanCoordinate, EuclideanDim>)],
    );
    let where_clause = &generics.where_clause;
    quote! {
        impl #generics Euclidean<EuclideanCoordinate, EuclideanDim> for #name #forward_generics
        #where_clause
        {
        }
    }
}

#[cfg(test)]
mod tests {
    use super::euclidean;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn euclidean_works_with_generics() {
        let input = parse_quote! {
            #[derive(Euclidean)]
            pub struct Hello<G, T, const D: usize>
            where
                G: Generator<[T; D]> + Euclidean<T, D>,
                T: Coordinate,
            {
                #[wrap]
                generator: G,
            }
        };

        let expected = quote! {
            impl<G, T, const D: usize, EuclideanCoordinate: topogen::traits::Coordinate, const EuclideanDim: usize> Euclidean<EuclideanCoordinate, EuclideanDim> for Hello<G, T, D>
            where
                G: Generator<[T; D]> + Euclidean<T, D>,
                T: Coordinate,
                G: Euclidean<EuclideanCoordinate, EuclideanDim>
            {
            }
        };
        let output = euclidean(input);
        assert_eq!(expected.to_string(), output.to_string());
    }

    #[test]
    fn euclidean_works_without_generics() {
        let input = parse_quote! {
            #[derive(Euclidean)]
            pub struct Hello {
                #[wrap]
                generator: Point<f64, 2>,
            }
        };

        let expected = quote! {
            impl<EuclideanCoordinate: topogen::traits::Coordinate, const EuclideanDim: usize> Euclidean<EuclideanCoordinate, EuclideanDim> for Hello
            where
                Point<f64, 2>: Euclidean<EuclideanCoordinate, EuclideanDim>
            {
            }
        };
        let output = euclidean(input);
        assert_eq!(expected.to_string(), output.to_string());
    }
}
