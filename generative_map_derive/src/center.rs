use crate::helpers::{create_forward_generics, expand_generics, find_wrapped};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

pub fn center(input: DeriveInput) -> TokenStream {
    let name = &input.ident;
    let wrapped = find_wrapped(&input).expect("Failed to find a field with the #[wrap] attribute");
    let wrapped_id = &wrapped.ident;
    let wrapped_ty = &wrapped.ty;
    let forward_generics = create_forward_generics(&input);
    let generics = expand_generics(
        &input,
        &[
            parse_quote!(EuclideanCoordinate: Coordinate),
            parse_quote!(const EuclideanDim: usize),
        ],
        &[
            parse_quote!(#wrapped_ty: Euclidean<EuclideanCoordinate, EuclideanDim> + Center<EuclideanCoordinate, EuclideanDim>),
        ],
    );
    let where_clause = &generics.where_clause;
    quote! {
        impl #generics Center<EuclideanCoordinate, EuclideanDim> for #name #forward_generics
        #where_clause
        {
            fn center(&self) -> [EuclideanCoordinate; EuclideanDim] {
                self. #wrapped_id .center()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::center;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn center_works_with_generics() {
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
            impl<G, T, const D: usize, EuclideanCoordinate: Coordinate, const EuclideanDim: usize> Center<EuclideanCoordinate, EuclideanDim> for Hello<G, T, D>
            where
                G: Generator<[T; D]> + Euclidean<T, D>,
                T: Coordinate,
                G: Euclidean<EuclideanCoordinate, EuclideanDim> + Center<EuclideanCoordinate, EuclideanDim>
            {
                fn center(&self) -> [EuclideanCoordinate; EuclideanDim] {
                    self.generator.center()
                }
            }
        };
        let output = center(input);
        assert_eq!(expected.to_string(), output.to_string());
    }
}
