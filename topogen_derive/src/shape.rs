use crate::helpers::{create_forward_generics, expand_generics, find_wrapped};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, DeriveInput};

pub fn shape(input: DeriveInput) -> TokenStream {
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
        &[parse_quote!(#wrapped_ty: Shape<EuclideanCoordinate, EuclideanDim>)],
    );
    let where_clause = &generics.where_clause;
    quote! {
        impl #generics Shape<EuclideanCoordinate, EuclideanDim> for #name #forward_generics
        #where_clause
        {
            fn distance(&self, point: [EuclideanCoordinate; EuclideanDim]) -> EuclideanCoordinate
            where EuclideanCoordinate: num_traits::Float {
                self. #wrapped_id .distance(point)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::shape;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn shape_works_with_generics() {
        let input = parse_quote! {
            #[derive(Euclidean)]
            pub struct Hello<S, T, const D: usize>
            where
                S: Shape<[T; D]>,
                T: Coordinate,
            {
                #[wrap]
                generator: S,
            }
        };

        let expected = quote! {
            impl<S, T, const D: usize, EuclideanCoordinate: Coordinate, const EuclideanDim: usize> Shape<EuclideanCoordinate, EuclideanDim> for Hello<S, T, D>
            where
                S: Shape<[T; D]>,
                T: Coordinate,
                S: Shape<EuclideanCoordinate, EuclideanDim>
            {
                fn distance(&self, point: [EuclideanCoordinate; EuclideanDim]) -> EuclideanCoordinate
                where EuclideanCoordinate: num_traits::Float {
                    self.generator.distance(point)
                }
            }
        };
        let output = shape(input);
        assert_eq!(expected.to_string(), output.to_string());
    }
}
